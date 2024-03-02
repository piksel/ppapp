import { Socket } from 'socket.io-client';
import { FC, useMemo } from 'react';
import { CurrentRound, User, Vote } from '../types';
import { InlineUser } from './InlineUser';
import { getScoreMoji } from '../voting';

interface Props { socket: Socket; users: User[]; userId: string | undefined; votes: Vote[]; round: CurrentRound | undefined; }
export const Votes: FC<Props> = (props) => {
    const flipped = !!props.round?.flipped;


    if (!flipped) {
        return <PendingVotes {...props} />
    }

    if (props.round?.anonymous) {
        return <VoteSummary {...props} />
    }

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
                        {getScoreMoji(voted ? flipped ? vote?.score ?? '' : 'cardback' : 'picking')}
                    </div>
                    <span className='text-white'>
                        <InlineUser user={u} userId={props.userId} />
                    </span>

                </li>);
            })}
        </ul>);
};

const PendingVotes: FC<Props> = (props) => {
    const flipped = !!props.round?.flipped;


    return (
        <ul className='flex flex-wrap justify-evenly gap-3'>
            {props.users.map(u => {
                const vote = props.votes.find(v => v.userID === u.userID);
                const voted = !!vote;
                const color = voted ? 'bg-orange-500' : 'bg-slate-800 bg-opacity-50';
                return (<li className='flex flex-col' key={u.userID}>
                    <div className={`${color} justify-center items-center font-bold text-4xl w-24 h-36 text-white rounded-md flex`}
                        aria-busy={!voted} aria-describedby={`progress-${u.userID}`}
                    >
                        {getScoreMoji(voted ? flipped ? vote?.score ?? '' : 'cardback' : 'picking')}
                    </div>
                    <span className='text-white'>
                        <InlineUser user={u} userId={props.userId} />
                    </span>

                </li>);
            })}
        </ul>);
};

const VoteSummary: FC<Props> = (props) => {

    const votes = useMemo(() => props.votes.sort((a, b) => {
        let na = parseInt(a.score);
        let nb = parseInt(b.score);
        if (isNaN(na)) {
            na = a.score.charCodeAt(0);
        }
        if (isNaN(nb)) {
            nb = b.score.charCodeAt(0);
        }
        return na - nb;
    }), [props.votes])

    return (
        <ul className='flex flex-wrap justify-evenly gap-3'>
            {votes.map((v, i) => {

                const color = 'bg-slate-600';
                return (<li className='flex flex-col' key={i}>
                    <div className={`${color} justify-center items-center font-bold text-4xl w-24 h-36 text-white rounded-md flex`}

                    >
                        {getScoreMoji(v.score ?? '')}
                    </div>

                </li>);
            })}
        </ul>);
};