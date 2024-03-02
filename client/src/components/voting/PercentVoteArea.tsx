import { Socket } from 'socket.io-client';
import { FC } from 'react';
import { Room, Vote } from '../types';
import { getScoreMoji } from '../voting';



export const VoteArea: FC<{ socket: Socket; room: Room; vote: Vote | undefined; candidates: string[] }> = (props) => {

    const doVote = (score: string) => {
        console.log('Voting for %o in %o', score, props.room.roomID);
        props.socket.emit('vote', { score, room: props.room.roomID });
    };

    const candidates = props.candidates ?? [];

    return <div>
        <div className='flex gap-2 flex-wrap'>
            {candidates.map(score => {

                const color = props.vote?.score === score ? 'bg-ctp-blue' : 'bg-slate-600';
                return <button key={score}
                    className={`${color} px-6 font-bold text-4xl w-24 h-36 text-white rounded-md inline-block`}
                    onClickCapture={() => doVote(score)}>{getScoreMoji(score)}</button>;

            })}
        </div>
    </div>;
};
