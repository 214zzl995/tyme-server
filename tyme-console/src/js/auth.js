import { getMqttUser } from './fetch.js';
import { user, createSocket, closeSocket, mqttUser, guide } from './store.js';

export async function getSession() {
    const res = await fetch('/auth/session', { credentials: 'same-origin' });
    let sessionResponse = await res.json();
    if (sessionResponse.user_id !== '' && sessionResponse.user_id !== undefined) {

        var host = window.location.host;
        createSocket({
            url: `ws://${host}/c/ws`,
            node: 'player',
            mode: 'audio',
            debug: true,
            flushingTime: 0,
            reconnectDelay: 3000,
            binaryType: 'arraybuffer',
            onopen: () => {
                console.log("onopen");
            },
            onmessage: (/** @type {any} */ data) => {
                console.log("onmessage", data);
            },
            onerror: (/** @type {any} */ error) => {
                console.log("onerror", error);
            },
            onclose: (/** @type {any} */ event) => {
                console.log("onclose", event);
            },
        })

        //获取 mqtt 用户名
        getMqttUser().then((/** @type {any} */ res) => {
            mqttUser.set(res.user);
        });

        user.set(sessionResponse.user_id);
    } else {
        if (sessionResponse.guide === true) {
            guide.set(true);
        }
        user.set('');
    }
}

/**
 * @param {any} username
 * @param {any} password
 */
export async function postLogin(username, password) {
    const res = await fetch("/auth/login", {
        method: "POST",
        headers: {
            Accept: "application/json",
            "Content-Type": "application/json",
        },
        body: JSON.stringify({ username: username, password: password }),
    });
    return await res.json();
}

export async function getLogout() {
    const res = await fetch("/auth/logout", { credentials: 'same-origin' });

    let logoutResponse = await res.json();
    if (logoutResponse.result == "error") {
        // may want to return an error here

    } else {
        user.set('');
        closeSocket();
    }
}