import { Socket } from 'socket.io-client';
import { FC } from 'react';
import { CurrentRound, User, Vote } from '../types';
import { InlineUser } from './InlineUser';
import { scoreMojis } from '../voting';

export const Votes: FC<{ socket: Socket; users: User[]; userId: string | undefined; votes: Vote[]; round: CurrentRound | undefined; }> = (props) => {
    const flipped = !!props.round?.flipped;


    return (
        <ul className='flex flex-wrap justify-evenly gap-3'>
            {props.users.map(u => {
                const vote = props.votes.find(v => v.userID === u.userID);
                const voted = !!vote;
                const color = voted ? flipped ? 'bg-slate-600' : 'bg-orange-500' : 'bg-slate-800 bg-opacity-50';
                return (<li className='flex flex-col' key={u.userID}>
                    <div className={`${color} justify-center items-center font-bold text-4xl w-24 h-36 text-white rounded-md flex`}
                        aria-busy={!voted} aria-describedby={`progress-${u.userID}`}
                    >
                        {scoreMojis[voted ? flipped ? vote?.score ?? '' : 'cardback' : 'picking']}
                    </div>
                    <span className='text-white'>
                        <InlineUser user={u} userId={props.userId} />
                    </span>
                </li>);
            })}
        </ul>);
};
