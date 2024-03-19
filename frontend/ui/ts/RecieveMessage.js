const button = document.getElementById("submit");
var chat;

button.addEventListener("click", (e) => display_message(e), false);

async function display_message(e) {
  e.preventDefault();
  invoke("susbcribe_to_channel", { channel_id: 1 }).then((response) => {
    console.log(response.content);
  });
}

function onReady() {
  chat = document.getElementById("convo-chat");
}

window.addEventListener("load", () => onReady());

window.addEventListener("new_message", (event) => {
  console.log(event.payload);
});
