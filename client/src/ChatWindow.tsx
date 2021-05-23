import React from 'react';

export type Message = {
    id: number,
    text: string,
};

type Props = {
    messages: Message[],
    input: string,
    onChange: React.ChangeEventHandler<HTMLInputElement>,
};

function RenderMessage(message: Message) {
    return (
        <p key={message.id}>{message.text}</p>
    );
}

function ChatWindow(props: Props) {
    return (
        <div className="ChatWindow">
            <div className="ChatWindow--Output">
                {props.messages.map(RenderMessage)}
            </div>
            <input className="ChatWindow--Input" value={props.input} onChange={props.onChange} />
        </div>
    );
}

export default ChatWindow;
