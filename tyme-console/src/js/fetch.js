import api from './request.js';


export const getSecure = (/** @type {{}} */ params) => api.get(`/secure`)

export const getAllTopic = (/** @type {{}} */ params) => api.get(`/c/get-all-topic`)

export const getConfig = (/** @type {{}} */ params) => api.get(`/c/config`)

export const putConfig = ( /** @type {any} */ body) => api.post('/c/config', body)

export const getApi = (/** @type {string} */ api_token) => api.get('/check', undefined, undefined, {
    'Authorization': 'Bearer ' + api_token,
})

export const uploadCrt = (/** @type {string} */ filename, /** @type {FormData} */ body) => api.post(`/c/upload-crt/${filename}`, body, "", "multipart/form-data")

export const getChatMsg = (/** @type {string} */ topic) => api.get(`/c/get-chat-msg/${encodeURIComponent(topic)}`)

export const sendMsg = (/** @type {any} */ params) => api.post(`/c/send`, params)

export const getMqttUser = () => api.get(`/c/get-mqtt-user`)

export const getAllTask = () => api.get(`/c/task`)

export const addTask = (/** @type {any} */ body) => api.post(`/c/task`, body)

export const getAllScriptFile = () => api.get(`/c/script-file-name`)

export const uploadScript = (/** @type {string} */ filename, /** @type {FormData} */ body) => api.post(`/c/upload-script/${filename}`, body, "", "multipart/form-data")
