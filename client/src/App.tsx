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
      input: "fads",
    };
  }

  onChangeInput(ev: React.ChangeEvent<HTMLInputElement>) {
    this.setState({
      input: ev.target.value,
    });
  }

  render() {
    return (
      <div className="App">
        <header className="App-header">
          <h1>Cognite chat</h1>
        </header>
        <ChatWindow messages={this.state.messages} input={this.state.input} onChange={this.onChangeInput.bind(this)} />
      </div>
    );
  }
}

export default App;
