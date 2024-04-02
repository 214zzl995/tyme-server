/**
 * WebSocket 封装
 * 主要功能：
 * 1. 自动重连；
 */

/**
 * @typedef {Object} Options
 * @property {string} url - WebSocket 请求地址
 * @property {'player'} node - 节点名称，只能是 'player'
 * @property {'audio'} mode - 模式名称，只能是 'audio'
 * @property {BinaryType} binaryType - WebSocket 的二进制类型
 * @property {boolean} [debug] - 是否启用调试模式
 * @property {number} [flushingTime] - 刷新时间
 * @property {number} [reconnectDelay] - 重新连接延迟时间
 * @property {function|undefined} [onopen] - 连接建立时的回调函数
 * @property {function|undefined} [onmessage] - 接收到消息时的回调函数
 * @property {function|undefined} [onerror] - 发生错误时的回调函数
 * @property {function|undefined} [onclose] - 连接关闭时的回调函数
 * @property {boolean|undefined} [isErrorReconnect] - 是否在错误时重新连接
 * @property {boolean|undefined} [heartbeat] - 是否启用心跳机制
 * @property {number|undefined} [heartbeatInterval] - 心跳超时时间
 */

export default class Socket {
    /**
     * WebSocket 实例
     * @type {WebSocket}
     */
    ws
    /**
     * 连接配置参数
     * @type {Options}
     */
    options
    /**
     * 错误消息队列
     * @type {any[]}
     */
    errorStack = []

    /**
    * 是否正在重连
    * @type {Boolean}
    */
    isReconnectLoading = false

    /**
     * 定时器ID
     * @type {any}
    */
    timeId = null

    /**
     * @param {Options} options
     */
    constructor(options) {
        this.options = options;
        this.init()
    }

    /**
     * 初始化 WebSocket
     */
    init() {
        if ('WebSocket' in window) {
            this.ws = new WebSocket(this.options.url);
            this.ws.binaryType = this.options.binaryType
            this.onOpen(this.options.onopen)
            this.onMessage(this.options.onmessage)
            this.onError(this.options.onerror)
            this.onClose(this.options.onclose)

            if (this.options.heartbeat) {
                this._sendPing();
            }
        } else {
            console.error('该浏览器不支持WebSocket!');
        }
    }

    // 获取 WebSocket 实例
    get getWebSocket() {
        return this.ws
    }

    /**
     * 设置连接成功后的回调函数
     * @param {*} cb
     */
    onOpen(cb) {
        this.ws.onopen = () => {
            console.log('websocket >>>> onOpen 连接成功!')
            // 发送成功连接之前所发送失败的消息
            this.errorStack.forEach(message => {
                this.ws.send(message)
            })
            cb && cb()
            this.errorStack = []
            this.isReconnectLoading = false
        }
    }

    /**
     * 设置接收 WebSocket 消息的回调函数
     * @param {*} cb 
     */
    onMessage(cb) {
        try {
            this.ws.onmessage = cb
        } catch (e) {
            console.error('error: ', e)
        }
    }

    /**
     * 设置连接失败后的回调函数
     * @param {*} cb 
     */
    onError(cb) {
        this.ws.onerror = (/** @type {any} */ err) => {
            console.error(err, 'websocket >>>> onError 连接异常!')
            cb && cb(err)
            if (!this.options.isErrorReconnect) return
            this.onReconnection()
            this.isReconnectLoading = false
        }
    }

    /**
     * 设置连接关闭后的回调函数
     * @param {*} cb 
     */
    onClose(cb) {
        this.ws.onclose = () => {
            console.log('websocket >>>> onClose 关闭连接!')
            // 用户手动关闭的不重连
            if (this.isCustomClose) return
            cb && cb()
            // this.onReconnection()
            this.isReconnectionLoading = false
        }
    }

    /**
     * 请求连接异常重连
     * @returns 
     */
    onReconnection() {
        // 重连请求延时
        const delay = this.options.reconnectDelay || 3000
        // 防止重复请求
        if (this.isReconnectionLoading) {
            console.log('websocket >>>> onReconnection 请勿重复请求连接!')
            return
        }
        console.log('websocket >>>> onReconnection 正在重连!')
        this.isReconnectionLoading = true
        clearTimeout(this.timeId)
        this.timeId = setTimeout(() => {
            this.init()
        }, delay)
    }

    /**
     * 手动发送请求
     * @param {*} message 
     * @returns 
     */
    handleSend(message) {
        // 连接失败时的处理
        if (this.ws.readyState !== WebSocket.OPEN) {
            console.error('websocket >>>> handleSend 请求发送失败!')
            this.errorStack.push(message)
            return
        }
        this.ws.send(message)
    }

    /**
     * 手动关闭连接
     */
    handleClose() {
        this.isCustomClose = true
        this.ws.close()
    }

    /**
     * 手动开启连接
     */
    handleStart() {
        this.isCustomClose = false
        this.onReconnection()
    }

    /**
     * 手动销毁连接实例
     */
    handleDestroy() {
        this.handleClose()
        this.ws = null
        this.errorStack = null
        console.log('websocket >>>> handleDestroy 实例已销毁!')
    }

    async _sendPing() {
        await new Promise((resolve) => setTimeout(resolve, this.options.heartbeatInterval | 60000));
        this.ws.send('ping');
        this._sendPing();
    }

}
