import React from 'react';

export type Message = {
    id: number,
    text: string,
};

type Props = {
    messages: Message[],
    input: string,
    onChange: React.ChangeEventHandler<HTMLInputElement>,
    sendMessage: (message: string) => void,
};

function RenderMessage(message: Message) {
    return (
        <p key={message.id}>{message.text}</p>
    );
}

function doSend(ev: React.FormEvent<HTMLFormElement>, props: Props) {
    ev.preventDefault();
    props.sendMessage(props.input);
}

function ChatWindow(props: Props) {
    return (
        <div className="ChatWindow">
            <div className="ChatWindow--Output">
                {props.messages.map(RenderMessage)}
            </div>
            <form onSubmit={(ev) => doSend(ev, props)}>
                <input placeholder="Say something…" className="ChatWindow--Input" value={props.input} onChange={props.onChange} />
            </form>
        </div>
    );
}

export default ChatWindow;
