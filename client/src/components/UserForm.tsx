import { FC, useEffect, useState } from "react";
import { Socket } from "socket.io-client";
import { User } from "../types";

interface Props { socket: Socket, user: User | undefined }
export const UserForm: FC<Props> = (props) => {
  const { socket, user } = props;
  const [name, setName] = useState(user?.name ?? "");
  const [email, setEmail] = useState(user?.email ?? "");
  const [cardback, setCardback] = useState("default");

  useEffect(() => {
    setName(user?.name ?? "");
    setEmail(user?.email ?? "");
  }, [user])

  const updateUser = () => {
    socket.emit("update user", {
      name,
      email,
      cardback
    })
  }

  return (
    <div className="bg-ctp-crust p-4 text-ctp-base flex gap-x-1">

      <div className="flex-1 p-1 flex flex-col text-left">
        <label htmlFor="user-name" className="text-slate-200">Name:</label>
        <input id="user-name"
          className="flex-1 p-2 rounded-md bg-ctp-text text-ctp-base placeholder-ctp-subtext0"
          placeholder="Name"
          type="text" value={name} onChange={e => setName(e.target.value)} />
      </div>

      <div className="flex-1 p-1 flex flex-col text-left">
        <label htmlFor="user-email" className="text-slate-200">Email:</label>
        <input id="user-email"
          className="flex-1 p-2 rounded-md bg-ctp-text text-ctp-base placeholder-ctp-subtext0"
          placeholder="E-mail"
          type="e-mail" value={email} onChange={e => setEmail(e.target.value)} />
      </div>

      <div className="flex-1 p-1 flex flex-col text-left">
        <label htmlFor="user-email" className="text-slate-200">Cardback:</label>
        <select id="user-cardback"
          className="flex-1 p-2 rounded-md bg-ctp-text text-ctp-base placeholder-ctp-subtext0"
          value={cardback} onChange={e => setCardback(e.target.value)}>
          <option value="default">Default</option>
        </select>
      </div>

      <div className="flex-auto p-1 flex flex-col text-left justify-end items-end">
        <button
          className="bg-ctp-blue px-6 font-bold text-ctp-base p-2 rounded-md"
          onClickCapture={() => updateUser()}>Update</button>

      </div>
    </div>
  )
}