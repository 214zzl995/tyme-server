import { writable, get } from 'svelte/store';
import Socket from './socket';
import { updateStorageValue } from './storage';

export const user = writable("");

export const guide = writable(false);

export const mqttUser = writable("");

export const socket = writable(undefined);

/** @type {import('svelte/store').Writable<Header>} */
export const topicActive = writable(undefined);

/**
 * @typedef {Object} Header
 * @property {string} id - The topic id.
 * @property {string} topic - The topic string.
 * @property {number} qos - The QoS value.
 */
export const changeActiveTopic = (/** @type {Header} */ topic) => {
    if (get(topicActive) === undefined || get(topicActive).id !== topic.id) {
        topicActive.set(topic);
        updateStorageValue("topic-active", topic);
    }
}

export const createSocket = (/** @type {import('./socket').Options} */ options) => {
    socket.set(new Socket(options));
}

export const closeSocket = () => {
    socket.update((s) => {
        if (s) {
            s.handleClose();
        }
        return undefined;
    });
}

//登陆后保持 websocket 连接

/** @type import('svelte/store').Writable<{id:number,type:"primary" | "secondary" | "tertiary" | "error",dismissible:Boolean,message:string,timeout:number}[]> */
export const toasts = writable([]);


export const dismissToast = (/** @type {number} */ id) => {
    console.log("dismissToast", id);
    toasts.update((all) => all.filter((t) => t.id !== id));
};

export const PrimaryToast = (/** @type {string} */ message) => {
    addToast({
        type: "primary", message,
        dismissible: true,
        timeout: 3000,
    });
}

export const SecondaryToast = (/** @type {string} */ message) => {
    addToast({
        type: "secondary",
        message,
        dismissible: true,
        timeout: 3000,
    });
}

export const TertiaryToast = (/** @type {string} */ message) => {
    addToast({
        type: "tertiary",
        message,
        dismissible: true,
        timeout: 3000,
    });
}

export const ErrorToast = (/** @type {string} */ message) => {
    addToast({
        type: "error",
        message,
        dismissible: true,
        timeout: 3000,
    });
}


const addToast = async (/** @type {{ timeout?: any; id?: number; type?: "primary" | "secondary" | "tertiary" | "error" , dismissible?: boolean; message: string; }} */ toast) => {
    return new Promise((resolve, reject) => {
        // Create a unique ID so we can easily find/remove it
        // if it is dismissible/has a timeout.
        const id = Math.floor(Math.random() * 10000);

        // Setup some sensible defaults for a toast.
        const defaults = {
            id,
            /** @type {"secondary"} */
            type: "secondary",
            dismissible: true,
            timeout: 3000,
        };

        // Push the toast to the top of the list of toasts
        toasts.update((all) => [{ ...defaults, ...toast }, ...all]);

        // If toast is dismissible, dismiss it after "timeout" amount of time.
        console.log("addToast", toast.timeout);
        if (toast.timeout) {
            setTimeout(() => {
                dismissToast(id);
                resolve(); // Resolve the promise when the timeout is done
            }, toast.timeout);
        } else {
            resolve(); // Resolve immediately if there's no timeout
        }
    });
};