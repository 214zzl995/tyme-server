import api from './request.js';

export async function getSecure() {
    let res = await fetch('/secure', { credentials: 'same-origin' });
    let secureResponse = await res.json();
    return JSON.stringify(secureResponse.session);
}



export const getConfig = (/** @type {{}} */ params) => api(`/c/config`, params)



export async function sendTest() {
    var myHeaders = new Headers();
    myHeaders.append("Content-Type", "application/json");
    var raw = JSON.stringify({
        "topic": "hello",
        "qos": 1,
        "content": {
            "type": "Text",
            "text": "Ma"
        }
    });

    await fetch("/c/send", {
        headers: myHeaders,
        method: 'POST',
        body: raw,
        redirect: 'follow',
        credentials: 'same-origin'
    })
        .then(response => response.text())
        .then(result => console.log(result))
        .catch(error => console.log('error', error));
}

/**
 * @param {string} api_token
 */
export async function getApi(api_token) {
    let res = await fetch('/check', {
        headers: {
            'Authorization': 'Bearer ' + api_token,
            Accept: "application/json",
        },
    });
    return await res.json();
} 