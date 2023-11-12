#include "./api.h"

void my_api(AsyncWebServer* server)
{
    SPIFFS.begin();
    server->on("/", HTTP_GET, [](AsyncWebServerRequest * request) {
        request->send(SPIFFS, "/index.html", "text/html");
    });

    server->on("/favicon.ico", HTTP_GET, [](AsyncWebServerRequest * request) {
        request->send(SPIFFS, "/favicon.ico", "image/x-icon");
    });

    server->on("/static/css/main.9fd7f6dd.css", HTTP_GET, [](AsyncWebServerRequest * request) {
        request->send(SPIFFS, "/static/css/main.9fd7f6dd.css", "text/css");
    });

    server->on("/static/js/main.8bf02666.js", HTTP_GET, [](AsyncWebServerRequest * request) {
        request->send(SPIFFS, "/static/js/main.8bf02666.js", "*/*");
    });

    server->on("/static/js/27.f3101c3e.chunk.js", HTTP_GET, [](AsyncWebServerRequest * request) {
        request->send(SPIFFS, "/static/css/27.f3101c3e.chunk.js", "*/*");
    });

    server->on("/manifest.json", HTTP_GET, [](AsyncWebServerRequest * request) {
        request->send(SPIFFS, "/manifest.json", "text/json");
    });

    server->begin();
}
