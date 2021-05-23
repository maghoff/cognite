import React from 'react';
import './App.css';
import ChatWindow from './ChatWindow';
import { Message } from './ChatWindow';

enum Status {
  LOADING,
  RUNNING,
};

type State =
  | {
    status: Status.LOADING,
  }
  | {
    status: Status.RUNNING,
    messages: Message[],
    input: string,
  };

class App extends React.Component {
  state: State;

  constructor(props: any) {
    super(props);
    this.state = { status: Status.LOADING };
  }

  async componentDidMount() {
    try {
      const res = await fetch("/messages");
      const json = await res.json();
      this.setState({
        status: Status.RUNNING,
        messages: json,
        input: "",
      })
    }
    catch (ex) {
      alert(`Unhandled exception\n\n${ex}`);
    }
  }

  onChangeInput(ev: React.ChangeEvent<HTMLInputElement>) {
    this.setState({
      input: ev.target.value,
    });
  }

  sendMessage(message: string) {
    if (this.state.status !== Status.RUNNING) {
      throw new Error("sendMessage cannot be used until RUNNING");
    }

    this.setState({
      messages: this.state.messages.concat([{
        id: this.state.messages.length,
        text: message,
      }]),
      input: "",
    });
  }

  render() {
    let inner;
    if (this.state.status === Status.LOADING) {
      inner = (<p>Loading…</p>);
    } else {
      inner = (
        <ChatWindow
          messages={this.state.messages}
          input={this.state.input}
          onChange={this.onChangeInput.bind(this)}
          sendMessage={this.sendMessage.bind(this)}
        />
      );
    }

    return (
      <div className="App">
        <header className="App-header">
          <h1>Cognite chat</h1>
        </header>
        {inner}
      </div>
    );
  }
}

export default App;
