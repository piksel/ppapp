
import './App.css'
import { useSocket } from './socket';

import { Login } from './components/Login';
import { RoomForm } from './components/RoomForm';
import { UserForm } from './components/UserForm';
import { Socket } from 'socket.io-client';
import { FC } from 'react';
import { Room, Round, User, Vote } from './types'

function App() {

  const { socket, userID, room, user, users, rounds, votes, vote } = useSocket();

  return (
    <>
      {room ? <>
        <div className='flex'>
          <div className='flex-1'>
            <h3 className="text-xl text-slate-300 text-left">Room</h3>
            <h2 className='text-4xl text-left'>{room.name}</h2>
          </div>

          <div className='flex-1'>
            <h3 className="text-xl text-slate-300 text-left">User</h3>
            <UserForm socket={socket} user={user} />
          </div>
        </div>

        <div className='flex gap-5 mt-3'>

          <div className='flex-auto'>
            <h3 className="text-xl text-slate-300 text-left mt-5">Votes</h3>
            <div className="bg-ctp-crust p-4 text-ctp-base flex gap-x-1 flex-row justify-between">

              <div>
                <Votes socket={socket} users={users} userId={userID} votes={votes} />
              </div>


              <div>

                <button
                  className={`bg-slate-300 px-6 font-bold text-ctp-base p-2 rounded-md mt-5`}
                  onClickCapture={() => socket.emit('new round', room.roomID)}>End round</button>
              </div>
            </div>

            <h3 className="text-xl text-slate-300 text-left mt-5">Your vote</h3>
            <div className='bg-ctp-crust p-4 text-ctp-base'>

              <VoteArea socket={socket} room={room} vote={vote} />
            </div>

          </div>

          <div className='flex-1'>
            <h3 className="text-xl text-slate-300 text-left">Users</h3>
            <div className='bg-ctp-crust p-4 text-ctp-base'>

              <UsersList socket={socket} users={users} userId={userID} />
            </div>

            <h3 className="text-xl text-slate-300 text-left mt-5">Rounds</h3>
            <div className='bg-ctp-crust p-4 text-ctp-base'>

              <RoundsList socket={socket} rounds={rounds} users={users} userId={userID} />
            </div>

          </div>
        </div>



      </> : userID ? <div>
        <pre>{JSON.stringify(userID, null, 4)}</pre>
        <RoomForm socket={socket} />
      </div> :
        (
          <Login socket={socket} />
        )
      }

    </>
  )
}


const UsersList: FC<{ socket: Socket, users: User[], userId: string | undefined }> = (props) => {
  return (<ul>{props.users.map(u => <li key={u.userID} className='text-white text-left'>
    <span>{u.name}</span>
    {u.userID === props.userId ? ' (you)' : ''}
  </li>
  )}</ul>)
}

const RoundsList: FC<{ socket: Socket, rounds: Round[], users: User[], userId: string | undefined }> = (props) => {
  const { rounds, users, userId } = props;
  return (<ul>

    {rounds.map((r, i) => <li key={i} className='text-white text-left pl-4'>
      <span>{r.name}</span>
      {': '}<ul className='pl-4'>{r.votes.map(v => {
        return <li key={v.userID}>
          {users.find(u => u.userID === v.userID)?.name ?? v.userID}
          {v.userID === userId ? ' (you)' : ''}
          {': '}
          {v.score}
        </li>
      })}</ul>
    </li>
    )}</ul>)
}

const Votes: FC<{ socket: Socket, users: User[], userId: string | undefined, votes: Vote[] }> = (props) => {



  return (<ul>{props.users.map(u => {
    const vote = props.votes.find(v => v.userID === u.userID);
    return (<li key={u.userID} className='text-white text-left'>
      <span>{u.name}</span>
      {u.userID === props.userId ? ' (you)' : ''}
      {': '}
      {vote?.score ?? <em>not voted</em>}
    </li>)
  }
  )}</ul>)
}

const VoteArea: FC<{ socket: Socket, room: Room, vote: Vote | undefined }> = (props) => {

  const doVote = (score: string) => {
    props.socket.emit('vote', { score, room: props.room.roomID });
  }

  return <div>
    <div>
      <em className='text-white'>{props.vote ? props.vote.score : 'You have not voted'}</em>
    </div>
    {scores.map((s, i) => {

      const round = i == 0 ? 'rounded-l-md' : i == scores.length - 1 ? 'rounded-r-md' : '';
      const color = props.vote?.score === s ? 'bg-ctp-blue' : 'bg-slate-600';
      return <button key={s}
        className={`${color} px-6 font-bold text-ctp-base p-2 ${round}`}
        onClickCapture={() => doVote(s)}>{s[0].toUpperCase() + s.substring(1)}</button>

    })}
  </div>
}

const namedScores = ['coffee', 'unknown', 'infinite'];
const scores = [...namedScores, ...[1, 2, 4, 8, 16].map(s => s + '')]


export default App
