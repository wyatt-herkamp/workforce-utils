document.getElementById("download").addEventListener("click", download);
var schedule = "";

function download() {
  if (schedule == "") {
    const sending = chrome.runtime.sendMessage({
      action: "getSchedule",
    });
    sending.then(
      (response) => {
        schedule = response.schedule;
        innerDownload();
      },
      (error) => {
        console.log(`Error: ${error}`);
      }
    );
  } else {
    innerDownload();
  }
}
function innerDownload() {
  console.log("Downloading: " + schedule);
  chrome.downloads.download({
    url: schedule,
    filename: "schedule.json", // Optional
  });
}

chrome.runtime.onMessage.addListener(function (msg, sender, sendResponse) {
  if (msg.action == "open_dialog_box") {
    alert("Message recieved!");
  }
});
