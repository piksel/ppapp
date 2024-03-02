import { Socket } from "socket.io-client";
import { FC } from "react";
import { Room, Vote } from "../../types";
import { getScoreMoji } from "../../voting";

interface Props {
  socket: Socket;
  room: Room;
  vote: Vote | undefined;
  candidates: string[];
}
export const PickOneVoteArea: FC<Props> = (props) => {
  const doVote = (score: string) => {
    console.log("Voting for %o in %o", score, props.room.roomID);
    props.socket.emit("vote", { score, room: props.room.roomID });
  };

  const candidates = props.candidates ?? [];

  return (
    <div>
      <div className="flex flex-wrap gap-2">
        {candidates.map((score) => {
          const color =
            props.vote?.score === score ? "bg-ctp-blue" : "bg-slate-600";
          return (
            <button
              key={score}
              className={`${color} inline-block h-36 w-24 rounded-md px-6 text-4xl font-bold text-white`}
              onClickCapture={() => doVote(score)}
            >
              {getScoreMoji(score)}
            </button>
          );
        })}
      </div>
    </div>
  );
};
