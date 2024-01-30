import { Socket } from 'socket.io-client';
import { FC } from 'react';
import { Round, User } from '../types';
import { InlineUser } from './InlineUser';
import { scoreMojis } from '../voting';

export const RoundsList: FC<{ socket: Socket; rounds: Round[]; users: User[]; userId: string | undefined; }> = (props) => {
    const { rounds, users, userId } = props;
    return (<ul className='flex flex-col'>

        {rounds.map((r, i) => <li key={i} className='text-white text-left'>
            <span>{r.name}</span>
            {': '}<ul className='pl-4'>{r.votes.map(v => {
                const user = users.find(u => u.userID === v.userID);
                return <li className='flex items-center' key={v.userID}>
                    <InlineUser userId={userId} user={user ?? { userID: v.userID, name: v.userID, avatar: v.userID, email: '' }} />
                    {': '}
                    {scoreMojis[v.score]}
                </li>;
            })}</ul>
        </li>
        )}
        {!rounds.length && <em className='text-white text-opacity-50'>No prior rounds</em>}
    </ul>);
};
