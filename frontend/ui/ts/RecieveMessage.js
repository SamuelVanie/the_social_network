const button = document.getElementById("submit");
const chat = document.getElementById('chat-bloc');

button.addEventListener('click', () => display_message());

async function display_message() {
    invoke('get_message', {}).then((response) => {
        let author = response.author;
        let content = response.message;
        let messageBloc = document.createElement('div');
        messageBloc.innerText = author + ': ' + content;
        chat.appendChild(messageBloc);
    })
}