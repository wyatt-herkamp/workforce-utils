let schedule = "";
function logResponse(request) {
  if (
    request.url.startsWith(
      "https://wft.homedepot.com/missioncontrol/v1/schedule/"
    )
  ) {
    schedule = request.url;
  }
}
function handleMessage(request, sender, sendResponse) {
  sendResponse({ schedule: schedule });
}

chrome.runtime.onMessage.addListener(handleMessage);

chrome.webRequest.onBeforeRequest.addListener(
  logResponse,
  {
    urls: ["https://wft.homedepot.com/*"],
  },
  ["extraHeaders", "requestBody"]
);
