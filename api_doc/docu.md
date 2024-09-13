# API

## Bvg

### Search for stops

```
curl 'https://v6.bvg.transport.rest/locations?poi=false&addresses=false&query=kottbussertor' -s
```
- return a list of stops with the given query

### Get departures from stop_id

```
curl 'https://v6.bvg.transport.rest/stops/900017101/departures?results=10'
```
- return a list of departures for the given stop
