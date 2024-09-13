// #![warn(clippy::implicit_return)]

use actix_web::dev::ServiceResponse;
use actix_web::{http, middleware, HttpResponse};
use chrono::{Local, Timelike};
use futures_util::future::ok;
use tokio::{net::UdpSocket, sync::Mutex};
use futures::future;
use actix_web::{dev::Service as _,web, App, HttpServer};
use future::{FutureExt, Future};
use uuid::Uuid;

use rpi_led_panel::{Canvas, HardwareMapping, RGBMatrix, RGBMatrixConfig};

use std::time::{self, Duration, SystemTime};
use std::thread;
use std::sync::Arc;
use std::pin::Pin;

use dotenv::dotenv;

use log::LevelFilter;
use log4rs::append::file::FileAppender;
use log4rs::encode::pattern::PatternEncoder;
use log4rs::config::{Appender, Config, Root};

mod http_endpoints;
mod led;
mod config;
mod cron;
mod fireplace;
mod util;
mod tokens;
mod pong;
pub mod api;
use crate::api::openstreetmap;
use crate::led::led::draw;
use crate::led::weather::show_weather_forecast;

pub type LedData = Option<LedJob>;
pub type JobHistory = Vec<LedJob>;

#[derive(Clone)]
pub struct LedJob {
    pub job_id: String,
    pub job_description: String,
    pub canvas: rpi_led_panel::Canvas,
    pub start_timestamp: std::time::SystemTime,
    pub stop_timestamp: std::time::SystemTime,
    pub canvas_update: Option<CanvasUpdate>,
    /// function specific data passed into the canvas update custom function
    pub led_func_data: LedFuncData,
    pub keep_prior_job_running: bool,
}

impl Default for LedJob {
    fn default() -> Self {
        let (_matrix, canvas) = 
            rpi_led_panel::RGBMatrix::new(get_rgb_matrix_config(), 0).expect("Matrix initialization failed");
        Self {
            job_id: Uuid::new_v4().to_string(),
            job_description: String::new(),
            canvas: *canvas,
            start_timestamp: SystemTime::now(),
            stop_timestamp: SystemTime::now() + std::time::Duration::from_secs(5 * 60),
            canvas_update: None,
            led_func_data: LedFuncData::default(),
            keep_prior_job_running: false,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct FireplaceData {
    time: f32,
    dark_rows: Vec<embedded_graphics::primitives::line::Line>,
    orange_rows: Vec<embedded_graphics::primitives::line::Line>,
    yelllow_rows: Vec<embedded_graphics::primitives::line::Line>,
}

#[derive(Clone, Debug)]
pub struct LedFuncData {
    pub position: Option<embedded_graphics::prelude::Point>,
    pub info_message: Option<String>,
    pub bvg_timetable: Option<Vec<crate::api::bvg::BvgTimeTable>>,
    pub autobahn_warn: Option<Vec<crate::api::autobahn::AutobahnWarning>>,
    pub weather_forcast: Option<crate::api::weatherkit::WeatherForcast>,
    pub weather_fercast_day: Option<u32>,
    pub osm_info: Option<openstreetmap::OpenStreetMapAddressdetails>,
    pub fireplace_data: Option<FireplaceData>,
    pub color: Option<String>,
    pub dashboard_widget: Option<crate::http_endpoints::DashboardWidgets>,
    pub football: Option<crate::api::football::FootballRes>,
    pub display_error_text: Option<String>,
    pub pong_game: Option<Arc<Mutex<pong::Game>>>,
}

impl Default for LedFuncData {
    fn default() -> Self {
        Self {
            position: None,
            info_message: None,
            bvg_timetable: None,
            autobahn_warn: None,
            weather_forcast: None,
            weather_fercast_day: None,
            osm_info: None,
            fireplace_data: None,
            color: None,
            dashboard_widget: None,
            football: None,
            display_error_text: None,
            pong_game: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum DelayType {
    /// in minutes
    Fixed(i32), 
    /// in std::time::Duration
    Random(std::time::Duration), 
}

type LedFunc = fn(LedFuncData, Box<Canvas>) -> Pin<Box<dyn Future<Output = (Canvas, LedFuncData)> + Send>>;

#[derive(Clone, Debug)]
pub struct CanvasUpdate {
    pub delay: DelayType,
    pub func: LedFunc,
}

pub struct AppState {
    auth_num: Mutex<http_endpoints::AuthNum>,
    led_data: Arc<Mutex<LedData>>,
    job_history: Arc<Mutex<JobHistory>>,
    pong_game: Arc<Mutex<pong::Game>>,
}


async fn udp_server() -> std::io::Result<()> {
    let sock = UdpSocket::bind("0.0.0.0:12345").await?;
    sock.set_broadcast(true)?;
    println!("Listening on [::]:12345");
    let mut buf = [0; 1024];
    loop {
        let (len, addr) = sock.recv_from(&mut buf).await?;
        println!("{:?} bytes received from {:?}", len, addr);
        log::info!("{:?} bytes received from {:?}", len, addr);
        let msg = String::from_utf8_lossy(&buf[..len]);
        log::info!("{:?} msg received from {:?}", msg, addr);

        let msg = "This is led-cat";
        let len = sock.send_to(msg.as_bytes(), addr).await?;
        println!("{:?} bytes sent", len);
    }
}

async fn https_server(led_data: Arc<Mutex<LedData>>, job_history: Arc<Mutex<JobHistory>>) -> std::io::Result<()> {
    let state = web::Data::new(AppState {
        auth_num: Mutex::new(http_endpoints::AuthNum { num: 0, created: SystemTime::now() }),
        job_history,
        led_data,
        pong_game: Arc::new(Mutex::new(pong::Game::new())),
    });

    let server =HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .route("/", web::get().to(http_endpoints::manual_hello))
            .service(
                web::scope("/api")
                .service(http_endpoints::show_clock)
                .service(http_endpoints::show_auth_num)
                .service(http_endpoints::verify_auth_num)
                .service(http_endpoints::check_token_endpoint)
                .service(http_endpoints::job_info)
                .service(http_endpoints::sys_info)
                .service(http_endpoints::delete_job)
                .service(http_endpoints::show_image)
                .service(http_endpoints::show_run_text)
                .service(http_endpoints::update_config)
                .service(http_endpoints::get_config)
                .service(http_endpoints::show_bvg_info)
                .service(http_endpoints::show_weather_info)
                .service(http_endpoints::update_scheduled_jobs)
                .service(http_endpoints::show_fireplace)
                .service(http_endpoints::show_football)
                .service(http_endpoints::show_dashboard)
            )
            .service(
                web::scope("/pong")
                .service(http_endpoints::pong_start_game)
                .service(http_endpoints::pong_player_up)
                .service(http_endpoints::pong_player_down)
            )
    })
    .bind(("0.0.0.0", 8080)).unwrap()
    .run()
    .await;

    Ok(())
}

pub fn get_rgb_matrix_config() -> rpi_led_panel::RGBMatrixConfig {
    rpi_led_panel::RGBMatrixConfig {
        cols: 64,
        rows: 64,
        chain_length: 1,
        parallel: 1,
        hardware_mapping: HardwareMapping::adafruit_hat_pwm(),
        ..Default::default()
    }
}

async fn led_service(led_data: Arc<Mutex<LedData>>, job_history: Arc<Mutex<JobHistory>>) -> std::io::Result<()> {
    let mut job: Option<LedJob> = None;
    let mut counter = 0;
    let mut minutes_passed = 0;
    let mut last_time_executed = SystemTime::now();
    let mut last_time_drawn = Local::now();

    // let rgb_matrix_config = get_rgb_matrix_config();
    // let mut position: Option<embedded_graphics::prelude::Point> = Some(embedded_graphics::prelude::Point::new(rgb_matrix_config.rows * 2, rgb_matrix_config.cols / 2));

    let (mut matrix, mut start_canvas) = 
        rpi_led_panel::RGBMatrix::new(get_rgb_matrix_config(), 0).expect("Matrix initialization failed");

    for step in 0.. {
        if counter == 100 {
            let mut mutex_guard = led_data.lock().await;

            if job.is_none() || mutex_guard.is_none() || (job.is_some() && mutex_guard.clone().is_some() && mutex_guard.clone().unwrap().job_id != job.clone().unwrap().job_id) {
                job = mutex_guard.clone();
            } else {
                *mutex_guard = job.clone();
            }

            counter = 0;
        }

        if Local::now().second() == 0 {
            minutes_passed += 1;
        }

        if job.is_some() {
            let stop_timestamp = job.clone().unwrap().stop_timestamp;
            if stop_timestamp < SystemTime::now() {
                job = None;
            } else {
                let mut delay_elapsed = false;
                if job.clone().unwrap().canvas_update.is_some() {
                    delay_elapsed = match job.clone().unwrap().canvas_update.clone().unwrap().delay {
                       DelayType::Fixed(value) => value <= minutes_passed,
                       DelayType::Random(duration) => duration <= last_time_executed.elapsed().unwrap(),
                    };
                }
                if job.clone().unwrap().canvas_update.is_some() && delay_elapsed {
                    // log::info!("delay elapsed");
                    let now = std::time::Instant::now();
                    let canvas_update_binding = job.clone().unwrap().canvas_update.unwrap();
                    let led_func_data_binding = job.clone().unwrap().led_func_data;
                    let (updated_canvas, updated_func_data) = 
                        (canvas_update_binding.func)(led_func_data_binding, start_canvas.clone()).await;
                    // log::info!("ran function in {}ms", now.elapsed().as_millis());

                    job = Some(LedJob {
                        canvas: updated_canvas,
                        led_func_data: updated_func_data,
                        ..job.clone().unwrap()
                    });

                    minutes_passed = 0;
                    last_time_executed = SystemTime::now();
                    counter = 99;
                    // log::info!("updated job in {}ms", now.elapsed().as_millis());
                } 
                draw(job.clone().unwrap().canvas, &mut matrix, &mut last_time_drawn);
            }
        } else {
            // wait 10 milis
            // last_time_executed = SystemTime::now();
            // minutes_passed = 0;
            thread::sleep(Duration::from_millis(10));
        }
        counter += 1;
    }
    Ok(())
}


#[tokio::main]
async fn main() {
    dotenv::from_path("/home/pi/.config/ledcat/.env");

    let logfile = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{d(%Y-%m-%d %H:%M:%S)} - {l} - {m}\n")))
        .build("/home/pi/.config/ledcat/output.log").expect("to work");

    let config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .build(Root::builder()
                   .appender("logfile")
                   .build(LevelFilter::Info)).expect("to work");

    log4rs::init_config(config).expect("to work");

    let led_data: Arc<Mutex<LedData>> = Arc::new(Mutex::new(Some(
        crate::LedJob { 
            job_id: Uuid::new_v4().to_string(),
            canvas: crate::led::start_screen::start_srceen().0,
            start_timestamp: SystemTime::now(),
            stop_timestamp: SystemTime::now() + std::time::Duration::from_secs(30),
            keep_prior_job_running: false,
            canvas_update: None,
            job_description: "show_start_srceen".to_string(),
            led_func_data: Default::default(),
        }
    )));

    let job_history = Arc::new(Mutex::new(vec![]));

    // let mut data = led_data.lock().await;
    // *data = Some(job.clone());

    tokio::spawn(udp_server());
    tokio::spawn(https_server(led_data.clone(), job_history.clone()));
    led_service(led_data.clone(), job_history.clone()).await;
}

