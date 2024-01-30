import { FC, useState } from "react";
import { Socket } from "socket.io-client";

interface Props { socket: Socket }
export const Login: FC<Props> = (props) => {
    const { socket } = props;
    const [name, setName] = useState("");

    const connect = () => {
        socket.auth = { username: name };
        socket.connect();
    }

    return (
        <>
            <h3 className="text-xl text-slate-300 text-left mt-5">First visit</h3>
            <div className="bg-ctp-crust p-8">
                <div className="text-white">
                    Please enter a name to begin:
                </div>
                <input
                    className="flex-1 p-2 rounded-l-md bg-ctp-text text-ctp-base placeholder-ctp-subtext0"
                    placeholder="Name"
                    type="text" value={name} onChange={e => setName(e.target.value)} />
                <button
                    className="bg-ctp-blue px-6 font-bold text-ctp-base p-2 rounded-r-md"
                    onClickCapture={() => connect()}>Play</button>
            </div>
        </>
    )
}