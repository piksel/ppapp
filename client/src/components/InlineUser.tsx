import { FC } from 'react';
import { User } from '../types';

export const InlineUser: FC<{ user: User; userId: string | undefined; nameless?: boolean; }> = ({ user, nameless, userId }) => {
    return <div className='flex items-center justify-center p-1'>
        <div className='rounded-md h-4 w-4 bg-slate-200 inline-block mr-2' style={{
            background: `url(https://gravatar.com/avatar/${user.avatar}?d=identicon)`,
            backgroundSize: 'cover'
        }}></div>
        {nameless ? undefined : <span className='text-ellipsis min-w-0 overflow-hidden block whitespace-break-spaces'>
            {user.name}
            {user.userID === userId ? <span className='text-sm ml-1'>(you)</span> : ''}
        </span>}
    </div>;
};
