import React from 'react';
import './App.css';
import ChatWindow from './ChatWindow';
import { Message } from './ChatWindow';

type State = {
  messages: Message[],
  input: string,
};

class App extends React.Component {
  state: State;

  constructor(props: any) {
    super(props);
    this.state = {
      messages: [
        { id: 0, text: "Hello, Joe!" }
      ],
      input: "",
    };
  }

  onChangeInput(ev: React.ChangeEvent<HTMLInputElement>) {
    this.setState({
      input: ev.target.value,
    });
  }

  sendMessage(message: string) {
    this.setState({
      messages: this.state.messages.concat([{
        id: this.state.messages.length,
        text: message,
      }]),
      input: "",
    });
  }

  render() {
    return (
      <div className="App">
        <header className="App-header">
          <h1>Cognite chat</h1>
        </header>
        <ChatWindow
          messages={this.state.messages}
          input={this.state.input}
          onChange={this.onChangeInput.bind(this)}
          sendMessage={this.sendMessage.bind(this)}
        />
      </div>
    );
  }
}

export default App;
