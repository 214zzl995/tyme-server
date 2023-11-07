import api from './request.js';


export const getSecure = (/** @type {{}} */ params) => api.get(`/secure`)

export const getAllTopic = (/** @type {{}} */ params) => api.get(`/c/get-all-topic`)

export const getConfig = (/** @type {{}} */ params) => api.get(`/c/config`)

export const putConfig = ( /** @type {any} */ body) => api.post('/c/config', body)

export const getApi = (/** @type {string} */ api_token) => api.get('/check', undefined, undefined, {
    'Authorization': 'Bearer ' + api_token,
})

export const uploadCrt = (/** @type {string} */ filename, /** @type {FormData} */ body) => api.post('/c/upload/' + filename, body, "", "multipart/form-data")

