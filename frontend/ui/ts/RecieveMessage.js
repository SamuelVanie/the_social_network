const button = document.getElementById("submit");
var chat;

async function display_message(e) {
  e.preventDefault();
  console.log("I call the subscribe function");
  invoke("susbcribe_to_channel", { channelId: 1 });
}

button.addEventListener("click", (e) => display_message(e), false);

listen("new_message", (message) => {
  console.log("I received a message");
  console.log(message);
});

function onReady() {
  chat = document.getElementById("convo-chat");
}

window.addEventListener("load", () => onReady());
