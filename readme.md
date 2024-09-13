# led-cat-server

## deployment:

### build:

- build raspian os target arm-unknown-linux-gnueabihf and in release mode
- using the rust cross compiler tool [cross](https://github.com/rust-embedded/cross)

```bash
cross build --target arm-unknown-linux-gnueabihf --release
```

### deployment:

```bash
scp -r target/arm-unknown-linux-gnueabihf/release/led-cat-server pi@[your_ip]:/home/pi
```

## Supported APIs:

- [x] [bvg](https://v6.bvg.transport.rest/getting-started.html)
- [x] [weatherKit](https://developer.apple.com/weatherkit/get-started/) bring your own api-key (included in apple developer account)
- [x] [openstreetmap](https://www.openstreetmap.org)
- [x] [Autobahn Info (autobahn gmbh)](https://verkehr.autobahn.de/)
- [ ] [Radio Info](https://myonlineradio.de/)
- [ ] [Fussball](https://api.openligadb.de/)
- [ ] [DWD Pollen](https://opendata.dwd.de/climate_environment/health/alerts/s31fg.json)

## Server-API Documentation:

- port :8080

### GET /api/show_auth_num

- generiert eine zufaellige vier stellige Auth. Nummer
- zeigt diese auf led matrix an (und in terminal)

### POST /api/verify_auth_num

- nimmt folgeneds body json:

```
{
    "auth_num": "1234"
}
```

- prueft ob die Auth. Nummer korrekt ist
- `{"status":"verify", "auth": "returns_your_auth_token"}` oder `{"status":"fail"}`

### GET /api/check_auth_token

- nimmt Bearer token

```
Authorization: Bearer <token>
```

- prueft ob der (vorher von `/api/verify_auth_num`) gesendete Auth. Token korrekt ist
- `{"status":"valid"}` oder `{"status":"unvalid"}`

### GET /api/sys_info

- nimmt Bearer token

```
Authorization: Bearer <token>
```

- system information

### POST /api/delete_job

- nimmt Bearer token
- deletet job aus mutex

### GET /api/job_info

- nimmt Bearer token
- gibt infos zu dem aktuellen job

### POST /api/show_image

```bash
curl -X POST \
-H "Content-Type: multipart/form-data" \
-H "Authorization: Bearer [your token]"
-F "file=@/Users/juliank/Downloads/nayncat.jpeg" \
http://192.168.1.120:8080/api/show_image
```

- nimmt Bearer token
- nimmt image file
- header `Content-Type: multipart/form-data`

### POST /api/show_run_text

```bash
curl -X POST \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer [your token]" \
  -d '{"text": "Hello\nWorld"}' \
  http://192.168.1.120:8080/api/show_run_text
```

- nimmt Bearer token
- nimmt body json mit text

### POST /api/config/update

- cronjob string see [here](https://crontab.guru/)

```json
{
  "bvg_stop_id": 900023232,
  "automation_auth_token": "<your token>",
  "schedules": [
    {
      "description": "bvg info um 6 uhr morgens",
      "job_name": "show_bvg_info",
      "cronjob": "0 6 * * *",
      "job_led_func_data": {
        "info_message": ""
      },
     {
          "description": "wetter info info um 6 uhr morgens",
          "job_name": "show_weather_info",
          "cronjob": "35 11 * * *",
          "job_led_func_data": {
            "info_message": ""
          },
          "osm_lat": 52.520008,
          "osm_lon": 13.404954,
        }
    }
  ]
}
```

- nimmt Bearer token
- nimmt body json mit config

### GET /api/config/get

- nimmt Bearer token
- gibt config

### POST /api/update_scheduled_jobs

- nimmt Bearer token
- liest scheduled jobs aus config
- schreibt sie in cronjob file

### POST /api/show_weather_info

- nimmt Bearer token
- nimmt body json mit location

1. Varitante: detail ansicht des heutigen tages:

```json
{
  "lat": 52.647449,
  "lon": 12.506484
}
```

2. Varitante: weniger detail reiche ansicht bis zu 10 tage in die Zukunft:

- tag von 0 (heute) bis 9

```json
{
  "lat": 52.647449,
  "lon": 12.506484,
  "day": 1
}
```

### POST /api/show_fireplace

- nimmt Bearer token
- nimmt body mit update speed in milisekunden delay

```json
{
  "speed": 50
}
```

### GET /api/show_dashboard

- nimmt Bearer token
- nimmt body mit wigdet

#### Widgets:

- `WeatherInfo`

```json
{
  "Widget": "WeatherInfo",
  "lat": 52.43596,
  "lon": 13.259834,
  "day": 0
}
```

- `AutobahnInfo`

```json
{
  "Widget": "AutobahnInfo",
  "road_id": "A100"
}
```
