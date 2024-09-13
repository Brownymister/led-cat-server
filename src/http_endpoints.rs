use actix_web::dev::ServiceRequest;
use actix_web::{get, post, web, HttpRequest, HttpResponse,  Responder, http::header::CONTENT_LENGTH};
use actix_multipart::Multipart;
use futures::{StreamExt, TryStreamExt};
use log::info;
use std::default;
use std::path::PathBuf;
use uuid::Uuid;

use tokio::fs;
use tokio::io::AsyncWriteExt;
use std::sync::Arc;

use mime::{ Mime, IMAGE_PNG, IMAGE_JPEG, IMAGE_GIF };
use image::{ DynamicImage, imageops::FilterType };
use embedded_graphics::prelude::Point;

use sysinfo::System;
use serde::{Serialize, Deserialize};
use std::fs::{File,OpenOptions};
use std::io::prelude::*;
use std::time::SystemTime;
use chrono::Local;
use rand::Rng;

use crate::api::api::{get_autobahn_info, get_bvg_info, get_osm_info, get_pollen_data, get_weather_info};
use crate::api::dwd::DwdPollen;
use crate::{fireplace, led, LedFuncData};
use crate::api::{autobahn, bvg::*};
use crate::api::autobahn::*;
use crate::api::football::*;
use crate::led::clock;
use crate::led::weather;
use crate::led::bvg;
use crate::led::run_text;
use crate::led::image as led_image;
use crate::led::dashboard;
// use crate::ble::get_temperature;
use crate::tokens::check_token;

#[derive(Deserialize)]
pub struct AuthNum {
    pub num: i32,
    pub created: SystemTime,
}

#[derive(Debug)]
pub struct FileUpload {
    pub filename: String,
    pub content: Vec<u8>,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(tag = "Widget")]
pub enum DashboardWidgets {
    WeatherInfo(WeatherInfoBody),
    AutobahnInfo(AutobahnInfoBody),
}


#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct AutobahnInfoBody {
    pub road_id: String,
    pub lat: f64,
    pub lon: f64,
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
pub struct BvgInfoBody {
    pub stop_id: u64,
    pub speed: u64,
}

#[derive(Deserialize)]
pub struct SpeedBody {
    pub speed: i32,
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
pub struct WeatherInfoBody {
    pub lat: f32,
    pub lon: f32,
    pub day: Option<u32>,
}

#[derive(Deserialize)]
struct VerificationNumberBody {
    auth_num: i32,
}

#[derive(Deserialize, Debug)]
struct RunTextBody {
    text: String,
    color: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct SystemInfo {
    total_memory: u64,
    used_memory: u64,
    total_swap: u64,
    used_swap: u64,
    system_name: String,
    kernel_version: String,
    os_version: String,
    host_name: String,
    cpu_count: usize,
}

#[derive(Serialize, Deserialize, Debug)]
struct JobInfo {
    status: String,
    job_id: String,
    job_description: String,
    job_timeout: String,
}

pub async fn manual_hello() -> impl Responder {
    return HttpResponse::Ok().body(format!("Hello, this is the led-cat!"));
}

#[get("/show_auth_num")]
pub async fn show_auth_num(data: web::Data<crate::AppState>) -> impl Responder {
    log::info!("Hit /api/show_auth_num");
    // randomize auth num
    let mut auth_num_unwr = data.auth_num.lock().await;
    let mut rng = rand::thread_rng();
    auth_num_unwr.num = rng.gen_range(1000..9999);
    auth_num_unwr.created = SystemTime::now();

    println!("auth num: {}", auth_num_unwr.num);
    log::info!("auth num: {}", auth_num_unwr.num);

    {
        let mut mutex_guard_job_history= data.job_history.lock().await;
        let mut data = data.led_data.lock().await;

        let canvas = clock::led_auth_number(&auth_num_unwr.num.to_string());
        let job = crate::LedJob { 
            job_id: Uuid::new_v4().to_string(),
            canvas, 
            start_timestamp: SystemTime::now(),
            stop_timestamp: SystemTime::now() + std::time::Duration::from_secs(5 * 60), 
            keep_prior_job_running: true,
            canvas_update: None,
            job_description: "auth_num".to_string(),
            led_func_data: Default::default(), 
        };
        *data = Some(job.clone());
        mutex_guard_job_history.push(job);
    }

    return HttpResponse::Ok().body("{\"status\": \"ok\"}");
}

#[get("/football")]
pub async fn show_football(req: HttpRequest,data: web::Data<crate::AppState>) -> impl Responder {
    log::info!("Hit /api/football");
    let auth_token = match get_bearer_token(&req) {
        Ok(num) => num,
        Err(e) => return HttpResponse::Ok().body(format!("{{\"status\": \"fail\", \"message\": \"{:?}\"}}", e)),
    };

    if check_token(&auth_token).is_err() {
        return HttpResponse::Ok().body("{\"status\": \"fail\"}");
    }

    let football_str = crate::api::api::get_football_data().await;
    log::info!("{}", football_str);
    let football: Option<FootballRes> = Some(match serde_json::from_str(&football_str) {
        Ok(bvg_timetable) => bvg_timetable,
        Err(e) => {
            log::info!("error with serde: {:?}", e);
            return HttpResponse::Ok().body(format!("{{\"status\": \"fail\", \"message\": \"{:?}\"}}", e))
        }
    });

    let led_func_data = crate::LedFuncData {
        football,
        ..Default::default()
    };

    let mut mutex_guard_job_history = data.job_history.lock().await;
    let mut data = data.led_data.lock().await;

    let date = Local::now();

    let (mut _matrix, mut canvas) = 
        rpi_led_panel::RGBMatrix::new(crate::get_rgb_matrix_config(), 0).expect("Matrix initialization failed");

    let canvas_update = crate::CanvasUpdate {
        delay: crate::DelayType::Random(std::time::Duration::from_secs(60)),
        // func: crate::led::football::show_football,
        func: |led_func_data, canvas| Box::pin(crate::led::football::show_football(led_func_data, canvas)),
    };
    let canvas = crate::led::football::show_football(led_func_data.clone(), canvas).await.0;

    let rgb_matrix_config = crate::get_rgb_matrix_config();
    let position = Some(Point::new((rgb_matrix_config.rows as i32) * 2 , (rgb_matrix_config.cols as i32) / 2));

    let job = crate::LedJob { 
        job_id: uuid::Uuid::new_v4().to_string(),
        canvas,
        start_timestamp: SystemTime::now(),
        stop_timestamp: SystemTime::now() + std::time::Duration::from_secs(60 * 10),
        keep_prior_job_running: true,
        canvas_update: Some(canvas_update),
        job_description: "show_football".to_string(),
        led_func_data
    };
    *data = Some(job.clone());
    mutex_guard_job_history.push(job);
    return HttpResponse::Ok().body("{\"status\": \"ok\"}");
}

#[post("/verify_auth_num")]
pub async fn verify_auth_num(data: web::Data<crate::AppState>, body: web::Json<VerificationNumberBody>) -> impl Responder {
    log::info!("Hit /api/verify_auth_num");
    let auth_num = body.auth_num;
    log::info!("auth num: {}", auth_num);

    let auth_num_is_current = SystemTime::now().duration_since(data.auth_num.lock().await.created).unwrap().as_secs() < 900;
    if auth_num != 0 && auth_num == data.auth_num.lock().await.num && auth_num_is_current{
        let token = crate::tokens::Token::new(None, Some("App authentication".to_string()));
        let res = token.save();
        if res.is_err() {
            log::info!("auth num verification faied: failed to create token");
            return HttpResponse::Ok().body("{\"status\": \"failed to create token\"}");
        }
        log::info!("successfully saved token");
        data.auth_num.lock().await.num = 0;

        let mut my_data = data.led_data.lock().await;
        *my_data = None;

        let unwr_res = res.unwrap();
        log::info!("auth num verified: {:?}", unwr_res);
        return HttpResponse::Ok().body(format!("{{\"status\": \"verified\", \"auth\":{:?}}}", unwr_res));
    }
    log::info!("auth num verification faied: prob. wrong auth num");
    return HttpResponse::Ok().body("{\"status\": \"fail\"}");
}

pub fn get_bearer_token(req: &HttpRequest) -> Result<String, std::io::Error>{
    let authorization_header = match req.headers().get("Authorization") {
        Some(header) => header,
        None => return Err(std::io::Error::new(std::io::ErrorKind::Other, "No bearer token provided")),
    };

    let bearer_token: Vec<&str> = authorization_header
       .to_str()
       .unwrap()
       .split_whitespace()
       .collect();

    if bearer_token[0] != "Bearer" {
        return Err(std::io::Error::new(std::io::ErrorKind::Other, "Invalid token"));
    }

    return Ok(bearer_token[1].to_string());
}

#[get("/check_auth_token")]
pub async fn check_token_endpoint(req: HttpRequest) -> impl Responder{
    log::info!("Hit /api/check_auth_token");
    let auth_token = match get_bearer_token(&req) {
        Ok(num) => num,
        Err(e) => {
            log::info!("login attempt failed, {:?}", e);
            return HttpResponse::Ok().body(format!("{{\"status\": \"fail\", \"message\": \"{:?}\"}}", e))
        }
        ,
    };
    log::info!("auth token: {}", auth_token);

    if check_token(&auth_token).is_ok() {
        log::info!("login attempt success: {}", auth_token);
        return HttpResponse::Ok().body("{\"status\": \"valid\"}");
    } else {
        log::info!("login attempt failed: {}", auth_token);
        return HttpResponse::Ok().body("{\"status\": \"unvalid\"}");
    }
}

#[get("/show_clock")]
pub async fn show_clock(req: HttpRequest, data: web::Data<crate::AppState>) -> impl Responder {
    log::info!("Hit /api/show_clock");
    let auth_token = match get_bearer_token(&req) {
        Ok(num) => num,
        Err(e) => return HttpResponse::Ok().body(format!("{{\"status\": \"fail\", \"message\": \"{:?}\"}}", e)),
    };
    println!("auth token: {}", auth_token);

    if check_token(&auth_token).is_err() {
        return HttpResponse::Ok().body("{\"status\": \"fail\"}");
    }
    let mut mutex_guard_job_history = data.job_history.lock().await;
    let mut data = data.led_data.lock().await;

    let date = Local::now();

    let canvas = clock::led_auth_number(&date.format("%H:%M").to_string());
    let canvas_update = crate::CanvasUpdate {
        delay: crate::DelayType::Fixed(1),
        func: |led_func_data, canvas| Box::pin(clock::led_clock(led_func_data, canvas)),
    };
    let job = crate::LedJob { 
        job_id: Uuid::new_v4().to_string(),
        canvas,
        start_timestamp: SystemTime::now(),
        stop_timestamp: SystemTime::now() + std::time::Duration::from_secs(5 * 60),
        keep_prior_job_running: false,
        canvas_update: Some(canvas_update),
        job_description: "show_clock".to_string(),
        led_func_data: Default::default(), 
    };
    *data = Some(job.clone());
    mutex_guard_job_history.push(job);

    return HttpResponse::Ok().body("{\"status\": \"ok\"}");
}

#[get("/sys_info")]
pub async fn sys_info(req: HttpRequest) -> impl Responder {
    log::info!("Hit /api/sys_info");
    let auth_token = match get_bearer_token(&req) {
        Ok(num) => num,
        Err(e) => return HttpResponse::Ok().body(format!("{{\"status\": \"fail\", \"message\": \"{:?}\"}}", e)),
    };

    if check_token(&auth_token).is_err() {
        return HttpResponse::Ok().body("{\"status\": \"fail\"}");
    } else {
        let mut sys = System::new_all();
        sys.refresh_all();
        let sys_info = SystemInfo {
            total_memory: sys.total_memory(),
            used_memory: sys.used_memory(),
            total_swap: sys.total_swap(),
            used_swap: sys.used_swap(),
            system_name: System::name().unwrap(),
            kernel_version: System::kernel_version().unwrap(),
            os_version: System::os_version().unwrap(),
            host_name: System::host_name().unwrap(),
            cpu_count: sys.cpus().len(),
        };

        let serialized_sys_info = serde_json::to_string(&sys_info).unwrap();
        return HttpResponse::Ok().body(serialized_sys_info);
    }
}

#[get("/job_info")]
pub async fn job_info(req: HttpRequest, data: web::Data<crate::AppState>) -> impl Responder {
    log::info!("Hit /api/job_info");
    let auth_token = match get_bearer_token(&req) {
        Ok(num) => num,
        Err(e) => return HttpResponse::Ok().body(format!("{{\"status\": \"fail\", \"message\": \"{:?}\"}}", e)),
    };

    if check_token(&auth_token).is_err() {
        return HttpResponse::Ok().body("{\"status\": \"fail\"}");
    }

    let mutex_guard = data.led_data.lock().await;

    if mutex_guard.clone().is_none() {
        return HttpResponse::Ok().body("{\"status\": \"no job\"}");
    }

    let job_id = mutex_guard.clone().unwrap().job_id;
    let job_description = mutex_guard.clone().unwrap().job_description;
    let job_timeout = mutex_guard.clone().unwrap().stop_timestamp;
    drop(mutex_guard);
    let job_timeout_utc: chrono::DateTime<chrono::Utc> = job_timeout.into();
    
    let job_info = JobInfo {
        status: "busy".to_string(),
        job_description,
        job_id,
        job_timeout: job_timeout_utc.format("%Y-%m-%d %H:%M:%S").to_string(),
    };

    let serialized_job_info = serde_json::to_string(&job_info).unwrap();
    log::info!("resoponded with job_info: {}", serialized_job_info);
    return HttpResponse::Ok().body(serialized_job_info);
}

#[post("/delete_job")]
pub async fn delete_job(req: HttpRequest, data: web::Data<crate::AppState>) -> impl Responder {
    let mut mutex_guard = data.led_data.lock().await;
    *mutex_guard = None;
    log::info!("deleted job");
    return HttpResponse::Ok().body("{\"status\": \"ok\"}");
}


pub async fn save_file(mut payload: Multipart, file_path: String) -> Option<bool> {
   log::info!("start saving file: {}", file_path);
   // iterate over multipart stream
   while let Ok(Some(mut field)) = payload.try_next().await {
       let content_type = field.content_disposition();
       //let filename = content_type.get_filename().unwrap();
       // let filepath = format!(".{}", file_path);
       println!("field : {:?}", field);
   }

   Some(true)
}

#[post("/show_image")]
pub async fn show_image(
    req: HttpRequest,
    mut payload: Multipart,
    data: web::Data<crate::AppState>,
) -> impl Responder {
    log::info!("Hit /api/show_image");
    let auth_token = match get_bearer_token(&req) {
        Ok(num) => num,
        Err(e) => return HttpResponse::Ok().body(format!("{{\"status\": \"fail\", \"message\": \"{:?}\"}}", e)),
    };

    if check_token(&auth_token).is_err() {
        return HttpResponse::Ok().body("{\"status\": \"fail\"}");
    }

    let content_length: usize = match req.headers().get(CONTENT_LENGTH) {
        Some(header_value) => header_value.to_str().unwrap_or("0").parse().unwrap(),
        None => "0".parse().unwrap(),
    };

    let max_file_count: usize = 1;
    let max_file_size: usize = 10_000_000;
    let legal_filetypes: [Mime; 3] = [IMAGE_PNG, IMAGE_JPEG, IMAGE_GIF];
    let mut current_count: usize = 0;
    let dir: &str = "/home/pi/.config/ledcat/images/";

    println!("content_length: {:#?}", content_length);
    println!("max_file_size: {:#?}", max_file_size);
    if content_length > max_file_size { return HttpResponse::Ok().body("{\"status\": \"fail\"}"); }

    loop {
        if current_count == max_file_count { break; }
        if let Ok(Some(mut field)) = payload.try_next().await {
            let filetype: Option<&Mime> = field.content_type();
            if filetype.is_none() { continue; }
            if !legal_filetypes.contains(&filetype.unwrap()) { continue; }

            println!("content_length: {:#?}", content_length);
            println!("{}. picture:", current_count);
            println!("name {}", field.name()); // &str
            println!("headers {:?}", field.headers());
            println!("content type {:?}", field.content_type()); // &Mime

            println!("filename {}", field.content_disposition().get_filename().unwrap()); // Option<&str>
            
            let destination: String = format!(
                "{}{}-{}",
                dir,
                Uuid::new_v4(),
                field.content_disposition().get_filename().unwrap()
            );

            let mut saved_file: fs::File = fs::File::create(&destination).await.unwrap();
            while let Ok(Some(chunk)) = field.try_next().await {
                let _ = saved_file.write_all(&chunk).await.unwrap();
            }

            web::block(move || async move {
                let uploaded_img: DynamicImage = image::open(&destination).unwrap();
                let _ = fs::remove_file(&destination).await.unwrap();
                uploaded_img
                    .resize_exact(64, 64, FilterType::Gaussian)
                    .save(format!("{}{}.gif", dir, "display_image".to_string())).unwrap();
            }).await.unwrap().await;
        } else { break; }
        current_count += 1;
    }

    {
        let mut mutex_guard_job_history = data.job_history.lock().await;
        let mut data = data.led_data.lock().await;

        let date = Local::now();

        let canvas = led_image::show_image("/home/pi/.config/ledcat/images/display_image.gif");
        let job = crate::LedJob { 
            job_id: Uuid::new_v4().to_string(),
            canvas,
            start_timestamp: SystemTime::now(),
            stop_timestamp: SystemTime::now() + std::time::Duration::from_secs(5 * 60),
            keep_prior_job_running: false,
            canvas_update: None,
            job_description: "show_image".to_string(),
            led_func_data: Default::default(),
        };
        *data = Some(job.clone());
        mutex_guard_job_history.push(job);
    }

    return HttpResponse::Ok().body("{\"status\": \"ok\"}");
}

#[post("/show_run_text")]
pub async fn show_run_text(req: HttpRequest, data: web::Data<crate::AppState>, body: web::Json<RunTextBody>) -> impl Responder{
    log::info!("Hit /api/show_run_text with body: {:?}", body);
    let auth_token = match get_bearer_token(&req) {
        Ok(num) => num,
        Err(e) => return HttpResponse::Ok().body(format!("{{\"status\": \"fail\", \"message\": \"{:?}\"}}", e)),
    };

    if check_token(&auth_token).is_err() {
        return HttpResponse::Ok().body("{\"status\": \"fail\"}");
    }

    let run_text = body.text.clone();
    if run_text.len() > 15 { return HttpResponse::Ok().body(format!("{{\"status\": \"fail\", \"message\": \"text too long; allowed: 15; your message: {} \"}}", run_text.len() )); }

    let mut mutex_guard_job_history = data.job_history.lock().await;
    let mut data = data.led_data.lock().await;

    let date = Local::now();

    let (mut _matrix, mut canvas) = 
        rpi_led_panel::RGBMatrix::new(crate::get_rgb_matrix_config(), 0).expect("Matrix initialization failed");

    let canvas_update = crate::CanvasUpdate {
        delay: crate::DelayType::Random(std::time::Duration::from_millis(10)),
        func: |led_func_data, canvas| Box::pin(run_text::led_run_text(led_func_data, canvas)),
    };

    let rgb_matrix_config = crate::get_rgb_matrix_config();
    let position = Some(Point::new((rgb_matrix_config.rows as i32) * 2 , (rgb_matrix_config.cols as i32) / 2));

    let mut led_func_data = crate::LedFuncData {
            position,
            info_message: Some(run_text.clone()),
            ..Default::default()
        } ;

    if body.color.is_some() {
        led_func_data.color = body.color.clone();
    }

    let job = crate::LedJob { 
        job_id: uuid::Uuid::new_v4().to_string(),
        canvas: *canvas,
        start_timestamp: SystemTime::now(),
        stop_timestamp: SystemTime::now() + std::time::Duration::from_secs(60),
        keep_prior_job_running: true,
        canvas_update: Some(canvas_update),
        job_description: "show_run_text".to_string(),
        led_func_data
    };
    *data = Some(job.clone());
    mutex_guard_job_history.push(job);
    info!("show_run_text: {}", run_text);
    return HttpResponse::Ok().body("{\"status\": \"ok\"}");
}

// #[get("/bvg_info")]
// pub async fn bvg_info(body: web::Json<BvgInfoBody>) -> impl Responder {
//     let stop_id = body.stop_id;
//     let body = get_bvg_info(stop_id).await;
//     return HttpResponse::Ok().body(body);
// }

#[post("/show_bvg_info")]
pub async fn show_bvg_info(body: web::Json<BvgInfoBody>, data: web::Data<crate::AppState>) -> impl Responder {
    log::info!("Hit /api/show_bvg_info with body: {:?}", body);
    let stop_id = body.stop_id;
    let bvg_info_str = get_bvg_info(stop_id).await;

    let bvg_timetable: BvgDeparture = match serde_json::from_str(&bvg_info_str) {
        Ok(bvg_timetable) => bvg_timetable,
        Err(e) => {
            log::info!("error with serde: {:?}", e);
            display_error_ledjob("Bvgdata not available".to_string(), data).await;
            return HttpResponse::Ok().body(format!("{{\"status\": \"fail\", \"message\": \"{:?}\"}}", e))
        },
    };
    log::info!("bvg timetable : {:?}", bvg_timetable);

    let led_func_data = crate::LedFuncData {
        position: Some(Point::new(crate::get_rgb_matrix_config().cols as i32 / 2, 10)),
        info_message: None,
        bvg_timetable: Some(bvg_timetable),
        ..Default::default()
    };

    let (_matrix, mut canvas) = 
        rpi_led_panel::RGBMatrix::new(crate::get_rgb_matrix_config(), 0).expect("Matrix initialization failed");
    let canvas = bvg::show_bvg_timetable(led_func_data.clone(), canvas).await.0;

    let job = crate::LedJob { 
        job_id: Uuid::new_v4().to_string(),
        canvas,
        start_timestamp: SystemTime::now(),
        stop_timestamp: SystemTime::now() + std::time::Duration::from_secs(60),
        keep_prior_job_running: false,
        canvas_update: Some(crate::CanvasUpdate {
            delay: crate::DelayType::Random(std::time::Duration::from_millis(body.speed)),
            func: |led_func_data, canvas| Box::pin(bvg::show_bvg_timetable(led_func_data, canvas)),
        }),
        job_description: "show_bvg_info".to_string(),
        led_func_data,
    };
    let mut mutex_guard = data.led_data.lock().await;
    *mutex_guard = Some(job);
    drop(mutex_guard);

    return HttpResponse::Ok().body("{\"status\": \"ok\"}");
}

#[post("/config/update")]
pub async fn update_config(req: HttpRequest, body: web::Json<crate::config::Config>) -> impl Responder {
    log::info!("Hit /api/config/update with body: {:?}", body);
    let auth_token = match get_bearer_token(&req) {
        Ok(num) => num,
        Err(e) => return HttpResponse::Ok().body(format!("{{\"status\": \"fail\", \"message\": \"{:?}\"}}", e)),
    };

    if check_token(&auth_token).is_err() {
        return HttpResponse::Ok().body("{\"status\": \"fail\"}");
    }

    crate::config::Config::update(body.0).unwrap();
    return HttpResponse::Ok().body("{\"status\": \"ok\"}");
}

#[get("/config/get")]
pub async fn get_config(req: HttpRequest) -> impl Responder {
    log::info!("Hit /api/config/get");
    let auth_token = match get_bearer_token(&req) {
        Ok(num) => num,
        Err(e) => return HttpResponse::Ok().body(format!("{{\"status\": \"fail\", \"message\": \"{:?}\"}}", e)),
    };

    if check_token(&auth_token).is_err() {
        return HttpResponse::Ok().body("{\"status\": \"fail\"}");
    }

    match crate::config::Config::new() {
        Ok(v) => return HttpResponse::Ok().json(v),
        Err(e) => return HttpResponse::Ok().body(format!("{{\"status\": \"fail\", \"message\": \"{:?}\"}}", e)),

    }
}

#[post("/update_scheduled_jobs")]
pub async fn update_scheduled_jobs(req: HttpRequest, data: web::Data<crate::AppState>) -> impl Responder {
    log::info!("Hit /api/update_scheduled_jobs");
    let auth_token = match get_bearer_token(&req) {
        Ok(num) => num,
        Err(e) => return HttpResponse::Ok().body(format!("{{\"status\": \"fail\", \"message\": \"{:?}\"}}", e)),
    };

    if check_token(&auth_token).is_err() {
        return HttpResponse::Ok().body("{\"status\": \"fail\"}");
    }

    let config = crate::config::Config::new().expect("config error");
    if config.clone().schedules.is_some() {
        crate::cron::delete_cronjob_file();
        for schedule_job in config.clone().schedules.unwrap() {
            crate::cron::new_con_job(schedule_job, &config.automation_auth_token, &config.bvg_stop_id)
        }
    }
    log::info!("updated jobs from config");
    return HttpResponse::Ok().body("{\"status\": \"updated\"}");
}

#[post("/show_weather_info")]
pub async fn show_weather_info(req: HttpRequest, body: web::Json<WeatherInfoBody>, data: web::Data<crate::AppState>) -> impl Responder {
    log::info!("Hit /api/show_weather_info");
    let auth_token = match get_bearer_token(&req) {
        Ok(num) => num,
        Err(e) => return HttpResponse::Ok().body(format!("{{\"status\": \"fail\", \"message\": \"{:?}\"}}", e)),
    };

    if check_token(&auth_token).is_err() {
        return HttpResponse::Ok().body("{\"status\": \"fail\"}");
    }

    if body.day.is_some() && body.day.unwrap() > 9 {
        log::info!("invalid forcast day: {}", body.day.unwrap());
        return HttpResponse::Ok().body("{\"status\": \"fail\"}, \"message\": \"invalid day\"}");
    }
 
    println!("show_weather_info: {:#?}", body);
    let weather_info_str = get_weather_info(body.lat, body.lon).await;
    let weather_timetable: crate::api::weatherkit::WeatherForcast = match serde_json::from_str(&weather_info_str) {
        Ok(weather_timetable) => weather_timetable,
        Err(e) => {
            display_error_ledjob("Weatherdata not available".to_string(), data).await;
            return HttpResponse::Ok().body(format!("{{\"status\": \"fail\", \"message\": \"{:?}\"}}", e))},
    };

    let osm_info_str = get_osm_info(body.lat, body.lon).await;
    println!("osm_info_str: {:#?}", osm_info_str);
    let osm_info: crate::openstreetmap::OpenStreetMapAddressdetails = match serde_json::from_str(&osm_info_str) {
        Ok(osm_info) => osm_info,
        Err(e) => {
            log::info!("osm_info error: {:?}", e);
            return HttpResponse::Ok().body(format!("{{\"status\": \"fail\", \"message\": \"{:?}\"}}", e))
        },
    };

    let led_func_data = crate::LedFuncData {
        position: Some(Point::new(crate::get_rgb_matrix_config().cols as i32 / 2, 10)),
        info_message: None,
        weather_forcast: Some(weather_timetable),
        weather_fercast_day: body.day,
        osm_info: Some(osm_info),
        ..Default::default()
    };

    let (_matrix, mut canvas) = 
        rpi_led_panel::RGBMatrix::new(crate::get_rgb_matrix_config(), 0).expect("Matrix initialization failed");
    let canvas = weather::show_weather_forecast(led_func_data.clone(), canvas).0;

    let job = crate::LedJob { 
        job_id: Uuid::new_v4().to_string(),
        canvas,
        start_timestamp: SystemTime::now(),
        stop_timestamp: SystemTime::now() + std::time::Duration::from_secs(10 * 60),
        keep_prior_job_running: false,
        job_description: "show_weather_info".to_string(),
        led_func_data,
        ..Default::default()
    };
    let mut mutex_guard = data.led_data.lock().await;
    *mutex_guard = Some(job);
    drop(mutex_guard);

    return HttpResponse::Ok().body("{\"status\": \"ok\"}");
}

#[post("/show_fireplace")]
pub async fn show_fireplace(req: HttpRequest, body: web::Json<SpeedBody>, data: web::Data<crate::AppState>) -> impl Responder {
    log::info!("Hit /api/show_fireplace");
    let auth_token = match get_bearer_token(&req) {
        Ok(num) => num,
        Err(e) => return HttpResponse::Ok().body(format!("{{\"status\": \"fail\", \"message\": \"{:?}\"}}", e)),
    };

    if check_token(&auth_token).is_err() {
        return HttpResponse::Ok().body("{\"status\": \"fail\"}");
    }

    let led_func_data = crate::LedFuncData {
        fireplace_data: Some(crate::FireplaceData { time: 0.0, dark_rows: Vec::new() , orange_rows: Vec::new(), yelllow_rows: Vec::new()}),
        ..Default::default()
    };

    let (_matrix, mut canvas) = 
        rpi_led_panel::RGBMatrix::new(crate::get_rgb_matrix_config(), 0).expect("Matrix initialization failed");
    let canvas = fireplace::show_fireplace(led_func_data.clone(), canvas).await.0;

    let job = crate::LedJob { 
        job_id: Uuid::new_v4().to_string(),
        canvas,
        start_timestamp: SystemTime::now(),
        stop_timestamp: SystemTime::now() + std::time::Duration::from_secs(10 * 60),
        keep_prior_job_running: false,
        job_description: "show_fireplace".to_string(),
        canvas_update: Some(crate::CanvasUpdate{
            delay: crate::DelayType::Random(std::time::Duration::from_millis(body.speed as u64)),
            func: |led_func_data, canvas| Box::pin(fireplace::show_fireplace(led_func_data, canvas)),
        }),
        led_func_data,
        ..Default::default()
    };
    let mut mutex_guard = data.led_data.lock().await;
    *mutex_guard = Some(job);
    drop(mutex_guard);

    return HttpResponse::Ok().body("{\"status\": \"ok\"}");
}


#[post("/show_dashboard")]
pub async fn show_dashboard(req: HttpRequest, body: web::Json<DashboardWidgets>, data: web::Data<crate::AppState>) -> impl Responder {
    log::info!("Hit /api/show_dashboard");
    let auth_token = match get_bearer_token(&req) {
        Ok(num) => num,
        Err(e) => return HttpResponse::Ok().body(format!("{{\"status\": \"fail\", \"message\": \"{:?}\"}}", e)),
    };

    if check_token(&auth_token).is_err() {
        return HttpResponse::Ok().body("{\"status\": \"fail\"}");
    }

    let dashboard_widget = body.clone();
    let mut weather_forcast = None;
    let mut autobahn_warn = None;
    match dashboard_widget {
        DashboardWidgets::AutobahnInfo(ref i) => {
            let autobahn_info_str = get_autobahn_info(i.road_id.clone()).await;
            println!("autobahn_info_str: {}", autobahn_info_str);
            log::info!("show_dashboard with Widget: autobahn_info");
            let parse_autobahn_info: crate::api::autobahn::AutobahnWarningsBody =  match serde_json::from_str(&autobahn_info_str) {
                Ok(autobahn_info) => autobahn_info,
                Err(e) => {
                    display_error_ledjob("Autobahndaten nicht vorhanden".to_string(), data).await;
                    return HttpResponse::Ok().body(format!("{{\"status\": \"failed to parse\", \"message\": \"{:?}\"}}", e))},
            };
            // autobahn_warn = Some(parse_autobahn_info);
            let lat = i.lat.round();
            let lon = i.lon.round();
            autobahn_warn = Some(parse_autobahn_info.warning
                .iter()
                .filter_map(|w| {
                    let round_lat = w.coordinate.lat.round();
                    let round_lon = w.coordinate.long.round();
                    println!("round_lat: {}, round_lon: {}, lat: {}, lon: {}, abnormalTrafficType: {:?}", round_lat, round_lon, lat, lon, (lat > (round_lat - 1.0) &&  lat < (round_lat + 1.0)) && (lon > (round_lon - 1.0) && lon < (round_lon + 1.0)));
                    let lat_check = lat == (round_lat - 1.0) || lat == (round_lat + 1.0) || lat == round_lat;
                    let lon_check = lon == (round_lon - 1.0) || lon == (round_lon + 1.0) || lon == round_lon;
                    if w.abnormalTrafficType.is_some() && lat_check && lon_check {
                        Some(w.clone())
                    } else {
                        None
                    }
                })
                .collect::<Vec<crate::api::autobahn::AutobahnWarning>>());
        },
        DashboardWidgets::WeatherInfo(ref i) => {
            log::info!("show_dashboard with Widget: weather_info");
            let weather_info_str = get_weather_info(i.lat, i.lon).await;
            let parse_weather_timetable: crate::api::weatherkit::WeatherForcast = match serde_json::from_str(&weather_info_str) {
                Ok(weather_timetable) => weather_timetable,
                Err(e) => {
                    display_error_ledjob("Weatherdata not available".to_string(), data).await;
                    return HttpResponse::Ok().body(format!("{{\"status\": \"fail\", \"message\": \"{:?}\"}}", e))},
            };
            weather_forcast = Some(parse_weather_timetable);
        },
        _ => {},
    };

    let position = Some(Point::new(32, 32 + 8));

    let led_func_data = crate::LedFuncData {
        position,
        dashboard_widget: Some(dashboard_widget.clone()),
        weather_forcast,
        autobahn_warn,
       ..Default::default()
    };
    println!("led_func_data: {:?}", led_func_data.clone());

    let (_matrix, mut canvas) = 
        rpi_led_panel::RGBMatrix::new(crate::get_rgb_matrix_config(), 0).expect("Matrix initialization failed");
    let mut canvas = dashboard::show_dashboard(led_func_data.clone(), canvas).await.0;

    let job = crate::LedJob { 
        job_id: Uuid::new_v4().to_string(),
        canvas,
        start_timestamp: SystemTime::now(),
        stop_timestamp: SystemTime::now() + std::time::Duration::from_secs(10 * 60),
        keep_prior_job_running: false,
        job_description: "show_dashboard".to_string(),
        canvas_update: Some(crate::CanvasUpdate{
            delay: crate::DelayType::Random(std::time::Duration::from_millis(50)),
            // func: dashboard::show_dashboard
            func: |led_func_data, canvas| Box::pin(dashboard::show_dashboard(led_func_data, canvas)),

        }),
        led_func_data,
        ..Default::default()
    };
    let mut mutex_guard = data.led_data.lock().await;
    *mutex_guard = Some(job);
    drop(mutex_guard);

    return HttpResponse::Ok().body("{\"status\": \"ok\"}");
}


async fn display_error_ledjob(msg: String, data: web::Data<crate::AppState>) {
    println!("display_error_ledjob: {:?}", msg);
    log::info!("Showing Error msg: {:?}", msg);
    let (_matrix, mut canvas) = 
        rpi_led_panel::RGBMatrix::new(crate::get_rgb_matrix_config(), 0).expect("Matrix initialization failed");

    let led_func_data = LedFuncData {
        display_error_text: Some(msg),
        ..Default::default()
    };

    let mut canvas = crate::led::error::display_error(led_func_data.clone(), canvas).0;
    let job = crate::LedJob {
        job_id: Uuid::new_v4().to_string(),
        canvas,
        start_timestamp: SystemTime::now(),
        stop_timestamp: SystemTime::now() + std::time::Duration::from_secs(1 * 60),
        job_description: "display_error_text".to_string(),
        led_func_data,
        ..Default::default()
    };
    let mut mutex_guard = data.led_data.lock().await;
    *mutex_guard = Some(job);
    drop(mutex_guard);
}


#[post("/player_up")]
async fn pong_player_up(state: web::Data<crate::AppState>) -> impl Responder {
    let mut mutex_guard_game = state.pong_game.lock().await;
    if  mutex_guard_game.running != true {
        return HttpResponse::Ok().body("{\"status\": \"err\", \"msg\":\"not running\"}");
    }
    if mutex_guard_game.player_up().is_err() {
        return HttpResponse::Ok().body("{\"status\": \"err\", \"msg\":\"reached top\"}");
    };
    mutex_guard_game.display();

    let res_j = format!("{{\"status\": \"ok\", \"board\":\"{}\"}}", mutex_guard_game.json());
    return HttpResponse::Ok().body(res_j);
}

#[post("/player_down")]
async fn pong_player_down(state: web::Data<crate::AppState>) -> impl Responder {
    let mut mutex_guard_game = state.pong_game.lock().await;
    if  mutex_guard_game.running != true {
        return HttpResponse::Ok().body("{\"status\": \"err\", \"msg\":\"not running\"}");
    }
    if mutex_guard_game.player_down().is_err() {
        return HttpResponse::Ok().body("{\"status\": \"err\", \"msg\":\"reached buttom\"}");
    };
    println!("");
    mutex_guard_game.display();

    let res_j = format!("{{\"status\": \"ok\", \"board\":\"{}\"}}", mutex_guard_game.json());
    return HttpResponse::Ok().body(res_j);
}

#[post("/start")]
async fn pong_start_game(state: web::Data<crate::AppState>) -> impl Responder {
    let pong_game_arc = Arc::clone(&state.pong_game);  // Clone the Arc for shared ownership

    let mut mutex_guard_game = pong_game_arc.lock().await;
    if mutex_guard_game.running {
        return HttpResponse::Ok().body("{\"status\": \"err\", \"msg\":\"already running\"}");
    }
    mutex_guard_game.running = true;
    drop(mutex_guard_game);  // Explicitly release the lock before continuing

    let led_func_data = crate::LedFuncData {
        pong_game: Some(pong_game_arc),  // Pass the cloned Arc<Mutex<PongGame>> to the struct
        ..Default::default()
    };

    let (_matrix, mut canvas) = 
        rpi_led_panel::RGBMatrix::new(crate::get_rgb_matrix_config(), 0).expect("Matrix initialization failed");
    let mut canvas = crate::led::pong::show_pong_led(led_func_data.clone(), canvas).await.0;

    let job = crate::LedJob { 
        job_id: Uuid::new_v4().to_string(),
        canvas,
        start_timestamp: SystemTime::now(),
        stop_timestamp: SystemTime::now() + std::time::Duration::from_secs(10 * 60),
        job_description: "show_pong_game".to_string(),
        canvas_update: Some(crate::CanvasUpdate {
            delay: crate::DelayType::Random(std::time::Duration::from_millis(30)),
            func: |led_func_data, canvas| Box::pin(crate::led::pong::show_pong_led(led_func_data, canvas)),
        }),
        led_func_data,
        ..Default::default()
    };
    let mut mutex_guard = state.led_data.lock().await;
    *mutex_guard = Some(job);
    drop(mutex_guard);

    return HttpResponse::Ok().body("{\"status\": \"ok\"}");
}

