import { FC, useEffect, useState } from "react";
import { Socket } from "socket.io-client";
import { User } from "../types";

interface Props { socket: Socket, user: User | undefined, onCloseDialog: () => void }
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
    });
    props.onCloseDialog();
  }

  const closeDialog = props.onCloseDialog;

  return (
    <div className="bg-ctp-crust p-4 text-ctp-base flex flex-col min-w-96 gap-2">

      <div className="flex-1 p-1 flex flex-col text-left">
        <label htmlFor="user-name" className="text-slate-200">Name:</label>
        <input id="user-name"
          className="flex-1 p-2 rounded-md bg-ctp-text text-ctp-base placeholder-ctp-subtext0"
          placeholder="Name"
          type="text" value={name} onChange={e => setName(e.target.value)} />
        <span className="text-slate-400 text-sm">This is the name that is displayed to the other players.</span>
      </div>

      <div className="flex-1 p-1 flex flex-col text-left">
        <label htmlFor="user-email" className="text-slate-200">Gravatar email:</label>
        <input id="user-email"
          className="flex-1 p-2 rounded-md bg-ctp-text text-ctp-base placeholder-ctp-subtext0"
          placeholder="E-mail"
          type="e-mail" value={email} onChange={e => setEmail(e.target.value)} />
        <span className="text-slate-400 text-sm">This e-mail address is only used to display an avatar next to your name.</span>
      </div>

      <div className="flex-1 p-1 flex flex-col text-left">
        <label htmlFor="user-email" className="text-slate-200">Cardback:</label>
        <select id="user-cardback"
          className="flex-1 p-2 rounded-md bg-ctp-text text-ctp-base placeholder-ctp-subtext0"
          value={cardback} onChange={e => setCardback(e.target.value)}>
          <option value="default">Default</option>
        </select>
        <span className="text-slate-400 text-sm">This is the back side of your card, displayed when you have voted.</span>
      </div>

      <div className="flex-auto p-1 flex flex-row text-left justify-end items-end gap-2">
        <button
          className="bg-slate-300 px-6 font-bold text-ctp-base p-2 rounded-md"
          onClickCapture={() => closeDialog()}>Cancel</button>
        <button
          className="bg-ctp-blue px-6 font-bold text-ctp-base p-2 rounded-md"
          onClickCapture={() => updateUser()}>Update</button>

      </div>
    </div>
  )
}