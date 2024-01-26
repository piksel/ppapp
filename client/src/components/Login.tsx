import { FC, useState } from "react";
import { Socket } from "socket.io-client";

interface Props { socket: Socket }
export const Login: FC<Props> = (props) => {
    const { socket } = props;
    const [username, setUsername] = useState("");

    const connect = () => {
        socket.auth = { username };
        socket.connect();
    }

    return (
        <>
            <h3 className="text-xl text-slate-300 text-left mt-5">Login</h3>
            <div className="bg-ctp-crust p-4">
                <input
                    className="flex-1 p-2 rounded-l-md bg-ctp-text text-ctp-base placeholder-ctp-subtext0"
                    placeholder="Username"
                    type="text" value={username} onChange={e => setUsername(e.target.value)} />
                <button
                    className="bg-ctp-blue px-6 font-bold text-ctp-base p-2 rounded-r-md"
                    onClickCapture={() => connect()}>Connect</button>
            </div>
        </>
    )
}