import { Socket } from 'socket.io-client';
import { FC } from 'react';
import { Room, Vote } from '../types';
import { scores, scoreMojis } from '../voting';



export const VoteArea: FC<{ socket: Socket; room: Room; vote: Vote | undefined; }> = (props) => {

    const doVote = (score: string) => {
        props.socket.emit('vote', { score, room: props.room.roomID });
    };

    return <div>
        <div className='flex gap-2 flex-wrap'>
            {scores.map(score => {

                const color = props.vote?.score === score ? 'bg-ctp-blue' : 'bg-slate-600';
                return <button key={score}
                    className={`${color} px-6 font-bold text-4xl w-24 h-36 text-white rounded-md inline-block`}
                    onClickCapture={() => doVote(score)}>{scoreMojis[score]}</button>;

            })}
        </div>
    </div>;
};
