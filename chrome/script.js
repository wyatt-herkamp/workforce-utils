document.getElementById("downloadSchedule").addEventListener("click", downloadSchedule);
var schedule = "";

function downloadSchedule() {
  if (schedule === "") {
    const sending = chrome.runtime.sendMessage({
      action: "getSchedule",
    });
    sending.then(
      (response) => {
        schedule = response.schedule;
        innerDownloadSchedule();
      },
      (error) => {
        console.log(`Error: ${error}`);
      }
    );
  } else {
    innerDownloadSchedule();
  }
}
function innerDownloadSchedule() {
  console.log("Downloading: " + schedule);
  chrome.downloads.download({
    url: schedule,
    filename: "schedule.json", // Optional
  });
}
