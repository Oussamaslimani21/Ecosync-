#include "./api.h"
#include "WiFi.h"
#include <esp_task_wdt.h>
int calculateSignalLevel(int rssi_level, int numLevels) {
    if (rssi_level <= -100) {
        return 0;
    } else if (rssi_level >= -55) {
        return numLevels - 1;
    } else {
        float inputRange = (-55 - (-100));
        float outputRange = (numLevels - 1);
        return (int)((float)(rssi_level - (-100)) * outputRange / inputRange);
    }
}

AsyncWebServer *server = new AsyncWebServer(80);

const char* ssid = "Me2";
const char* password =  "1337x.to";

void setup()
{
  Serial.begin(115200);
  WiFi.mode(WIFI_AP_STA);
  WiFi.softAP(ssid, password);
  Serial.println(WiFi.softAPIP());
  DefaultHeaders::Instance().addHeader("Access-Control-Allow-Origin", "*");
  server->on("/scan", HTTP_GET, [](AsyncWebServerRequest * request) {
    esp_task_wdt_init(30, false);
    int n = WiFi.scanNetworks();
    esp_task_wdt_init(5, true);
    String x = "";
    String y = "{\"others\":[";
    if (n != 0) {
      for (int i = 0; i < n; ++i){
        x += String(WiFi.SSID(i)) + " (" + String(calculateSignalLevel(WiFi.RSSI(i), 5)) + ")" + String((WiFi.encryptionType(i) == WIFI_AUTH_OPEN) ? "" : "*") + String(i == n - 1 ? "" : "\n");
        y += "{\"text\":\"" + String(WiFi.SSID(i)) + "\", \"sigStrength\":\"" + String(calculateSignalLevel(WiFi.RSSI(i), 5)) + "\", \"select\":\"";
        y += String((WiFi.encryptionType(i) == WIFI_AUTH_OPEN) ? "1" : "0") + "\"}" + String(i == n - 1 ? "]}" : ",");
    }}
    Serial.println("went");
    request->send(200, "application/json", y);
  });
  server->on("/connect", HTTP_POST, [](AsyncWebServerRequest * request) {
    //AsyncWebParameter* p = request->getParam(i);
    WiFi.begin(request->getParam(0)->value().c_str(), request->getParam(1)->value().c_str());
    request->send_P(200, "text/html", "Success");
  });
  my_api(server);
}

void loop()
{
  
}
