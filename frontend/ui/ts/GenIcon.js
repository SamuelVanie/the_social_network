
const button1 = document.getElementById("new-icon");
const body = document.getElementById("body");

if (button1!= null && body!=null) {
    
    button1.addEventListener('click', function () {invoke('icon', {})
    .then((response) => {
        div = document.createElement("div");
        itemPreview = document.createElement("img");
        body.appendChild(itemPreview);
        itemPreview.src = "data:image/png;base64," + _arrayBufferToBase64(response);
    })
    })
}


function _arrayBufferToBase64( buffer ) {
    var binary = '';
    var bytes = new Uint8Array( buffer );
    var len = bytes.byteLength;
    for (var i = 0; i < len; i++) {
        binary += String.fromCharCode( bytes[ i ] );
    }
    return window.btoa( binary );
}