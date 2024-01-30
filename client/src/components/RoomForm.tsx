import { FC, useState } from "react";
import { Socket } from "socket.io-client";

interface Props { socket: Socket }
export const RoomForm: FC<Props> = (props) => {
  const { socket } = props;
  const [roomId, setRoomId] = useState("");
  const [roomName, setRoomName] = useState("");

  const joinRoom = (roomId: string) => {
    console.log('Joining room %o', roomId);
    socket.emitWithAck("join", roomId).then(r => {
      console.log("Join successful: %o", r);
    }, e => {
      console.error('Join failed: %o', e);
    })
  }

  const createRoom = () => {
    console.log('Creating room %o', roomName);
    socket.emitWithAck("create room", roomName).then(r => {
      console.log("Created successful: %o", r);
      joinRoom(r.roomID);
    }, e => {
      console.error('Failed to create room: %o', e);
    })
  }

  return (<>
    <h3 className="text-xl text-slate-300 text-left mt-5">Join or create a room</h3>
    <div className="flex items-center justify-between">
      <div className="bg-ctp-crust p-4 flex-1">
        <input
          className="flex-1 p-2 rounded-l-md bg-ctp-text text-ctp-base placeholder-ctp-subtext0"
          placeholder="Room ID"
          type="text" value={roomId} onChange={e => setRoomId(e.target.value)} />
        <button
          className="bg-ctp-blue px-6 font-bold text-ctp-base p-2 rounded-r-md"
          onClickCapture={() => joinRoom(roomId)}>Join room</button>
      </div>
      <div className="text-white text-2xl p-6">or</div>
      <div className="bg-ctp-crust p-4 flex-1">
        <input
          className="flex-1 p-2 rounded-l-md bg-ctp-text text-ctp-base placeholder-ctp-subtext0"
          placeholder="Room name"
          type="text" value={roomName} onChange={e => setRoomName(e.target.value)} />
        <button
          className="bg-ctp-blue px-6 font-bold text-ctp-base p-2 rounded-r-md"
          onClickCapture={() => createRoom()}>Create new room</button>
      </div>
    </div>
  </>)
}