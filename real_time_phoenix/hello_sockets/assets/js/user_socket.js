import { Socket } from "phoenix"

const socket = new Socket("/socket", {})

socket.connect()

const channel = socket.channel("ping")

channel.join()
  .receive("ok", (res) => { console.log("joined ping", res) })
  .receive("error", (res) => { console.log("unable to join ping", res) })

console.log("send ping")
channel.push("ping")
  .receive("ok", (res) => console.log("receive", res.ping))

console.log("send pong")
channel.push("pong")
  .receive("ok", (res) => console.log("won't happen"))
  .receive("error", (res) => console.log("won't happen yet"))
  .receive("timeout", (res) => console.log("pong message timeout"))

channel.push("param_ping", { error: true })
  .receive("error", (res) => console.error("param_ping error:", res))

channel.push("param_ping", { error: false, data: [1, 2, 3] })
  .receive("ok", (res) => console.log("param_ping ok:", res))

channel.on("send_ping", (payload) => {
  console.log("ping requested", payload)
  channel.push("ping")
    .receive("ok", (res) => console.log("ping:", res.ping))
})

const authSocket = new Socket("/auth_socket", {
  params: { token: window.authToken }
})

authSocket.onOpen(() => console.log('authSocket connected'))
authSocket.connect()


export default socket
