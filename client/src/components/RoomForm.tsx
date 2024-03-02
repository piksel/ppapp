import { FC, useState } from "react";
import { Socket } from "socket.io-client";
import { Game, Games } from "../types/games";

interface Props { socket: Socket }
export const RoomForm: FC<Props> = (props) => {
  const { socket } = props;
  const [roomId, setRoomId] = useState("");
  const [roomName, setRoomName] = useState("");
  const [game, setGame] = useState<Game>('effort');

  const joinRoom = (roomId: string) => {
    console.log('Joining room %o', roomId);
    socket.emitWithAck("join", roomId).then(r => {
      console.log("Join successful: %o", r);
    }, e => {
      console.error('Join failed: %o', e);
    })
  }

  const createRoom = () => {
    console.log('Creating room %o with game %o', roomName, game);
    socket.emitWithAck("create room", roomName, game).then(r => {
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
      <div className="bg-ctp-crust p-4 flex-1 flex text-white text-left items-end">
        <div className="flex-1">
          <label htmlFor="roomName">Room name:</label>
          <input
            className="p-2 rounded-l-md bg-ctp-text text-ctp-base placeholder-ctp-subtext0"
            placeholder="Room name" name="roomName"
            type="text" value={roomName} onChange={e => setRoomName(e.target.value)} />
        </div>
        <div className="flex flex-col">
          <label htmlFor="game">Game:</label>
          <select value={game} name="game" onChange={(e) => setGame(e.target.value as Game)}
            style={{ padding: '0.6rem' }}
            className="flex-1 p-2 bg-ctp-text text-ctp-base placeholder-ctp-subtext0">
            {Object.entries(Games).map(([name, value]) => <option key={value} value={value}>{name}</option>)}
          </select>
        </div>

        <div className="min-w-fit">

          <button
            className="bg-ctp-blue px-6 font-bold text-ctp-base p-2 rounded-r-md"
            onClickCapture={() => createRoom()}>Create new room</button>
        </div>
      </div>
    </div>
  </>)
}