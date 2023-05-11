import { invoke } from "@tauri-apps/api/tauri";
import { listen } from "@tauri-apps/api/event";
import format from "date-fns/format";
import React from "react";


function App() {
  return (
    <div>
      <button onClick={() => invoke('hello1')} >Hello1</button>
      <button onClick={() => invoke('hello2', { msg: "jimmy" })} >Hello2</button>
      <button onClick={() => hello3()} >Hello3</button>
      <button onClick={() => get_person("lisa", 18)} >Get Person</button>
      <button onClick={() => method_1()}>method_1</button>
      <Home />
    </div>
  )
}

function hello3() {
  invoke('hello3', { msg: "lisa" }).then((res) => {
    console.log(res);
  })
}

const get_person = (name: string, age: Number) => {
  invoke('get_person', { name: name, age: age }).then((res) => {
    console.log(res);
  })
}

const method_1 = () => {
  invoke('method_1').then((res) => {
    console.log("result: ", res);
  })
}

interface Payload {
  message: Array<string>;
  timestamp: number;
}

let unlisten: any = null;

class Home extends React.Component {
  state = {
    message: [],
    timestamp: "",
    time: ""
  }

  start = () => {
    invoke('init_process');
    if (unlisten != null) {
      console.log("already listening");
      return;
    }

    const start_listen = async () => {
      return await listen<Payload>('my-event', (event) => {
        const { message, timestamp } = event.payload;
        console.log("message: ", message, "timestamp: ", timestamp,
          "time: ", format(new Date(timestamp), "yyyy-MM-dd HH:mm:ss"));
        this.setState({ message, timestamp, "time": format(new Date(timestamp), "yyyy-MM-dd HH:mm:ss") });
      });
    };

    unlisten = start_listen();
  }

  stop = () => {
    console.log("is_listening: ", unlisten != null);
    if (unlisten != null) {
      unlisten.then((ok: any) => {
        ok();
        unlisten = null;
        console.log("stop success");
      }).catch((err: any) => {
        console.log("stop failed", err);
      });
    }
  }

  render() {
    return (
      <div>
        <button onClick={this.start}>Start</button>
        <button onClick={this.stop}>Stop</button>
        <h4>{this.state.time}</h4>
        <div>
          {
            this.state.message.map((item, index) => {
              return (<span key={`${this.state.timestamp}_${index}`}>{item}</span>)
            })
          }
        </div>
      </div>
    )
  }
}



export default App;