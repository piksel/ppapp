import { Socket } from 'socket.io-client';
import { FC } from 'react';
import { User } from '../types';
import { InlineUser } from './InlineUser';

export const UsersList: FC<{ socket: Socket; users: User[]; userId: string | undefined; }> = (props) => {
    return (<ul className='text-white flex flex-col items-start'>
        {props.users.map(u => <li key={u.userID}>
            <InlineUser key={u.userID} userId={props.userId} user={u} />
        </li>
        )}</ul>);
};
