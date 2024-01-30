
import './App.css'
import { useSocket } from './socket';

import { Login } from './components/Login';
import { RoomForm } from './components/RoomForm';
import { UserForm } from './components/UserForm';
import { FC, useState } from 'react';
import { Result } from './types'
import { ToastsContainer } from './components/ToastsContainer';
import { UsersList } from './components/UsersList';
import { RoundsList } from './components/RoundsList';
import { Votes } from './components/Votes';
import { VoteArea } from './components/VoteArea';
import { Toast, ToastType } from './types/toasts';

function App() {
  const [showUserModal, setShowUserModal] = useState(false);
  const { socket, userID, room, user, users, rounds, votes, vote, currentRound } = useSocket();

  const [toasts, setToasts] = useState<Toast[]>([]);

  const addToast = (message: string, type: ToastType) => {
    setToasts(curr => [...curr, { message, type, id: crypto.randomUUID() }])
  }

  const removeToast = (id: string) => {
    setToasts(curr => curr.filter(t => t.id !== id));
  }

  const handleResult = (r: Result) => {
    if (r.type === 'Error') {
      addToast(r.error, 'error');
    }
  }

  const editCurrentRound = () => { /* TODO */ };


  return (
    <>
      <ToastsContainer toasts={toasts} removeToast={removeToast} />
      {user ? <>
        < div className='flex  justify-between gap-5' >
          <div className='flex-1'>
            {room && (<>
              <div className="text-lg bg-slate-800 px-4 py-2 text-slate-300 text-left rounded-t-md flex flex-row gap-3">
                <div className="text-xl text-slate-300 text-left">Room</div>

                <button className='underline text-white text-opacity-50 text-sm' onClickCapture={() => { }}>Edit</button>
              </div>
              <div className='bg-ctp-crust rounded-b-md py-1 px-4 text-4xl text-white text-left'>
                {room.name}
              </div>

            </>)

            }
          </div>

          <div className='bg-ctp-crust p-3 w-64 flex justify-stretch rounded-md'>
            <button className='flex flex-auto items-center gap-4 bg-slate-600 rounded-xl p-2' type='button' onClick={() => setShowUserModal(true)}>
              <div className='rounded-xl h-12 w-12 bg-slate-200' style={{
                background: `url(https://gravatar.com/avatar/${user.avatar}?d=identicon)`,
                backgroundSize: 'cover'
              }}></div>
              <div className="text-xl text-slate-300 text-left">{user.name}</div>
            </button>
          </div>
        </div >

        {room ? <>
          < div className='flex gap-5 mt-3' >



            <div className='flex-auto'>
              <div className="text-lg bg-slate-800 pl-4 pr-2 py-2 text-slate-300 text-left rounded-t-md">

                <div className='flex justify-between items-center'>

                  <h3 className=" text-slate-300 text-left">
                    <span className='text-xl'>{currentRound?.name ?? ''}</span>
                    <button className='underline text-white text-opacity-50 text-sm ml-2' onClickCapture={() => editCurrentRound()}>Edit</button>
                  </h3>
                  {currentRound?.flipped ? (
                    <button
                      className={`bg-slate-600 text-white px-6 py-1 rounded-md`}
                      onClickCapture={() => socket.emit('new round', room.roomID, handleResult)}>New round</button>
                  ) : (
                    <button
                      className={`bg-slate-600 text-white px-6 py-1 rounded-md`}
                      onClickCapture={() => socket.emit('end vote', room.roomID, handleResult)}>Reveal cards</button>
                  )}

                </div>
              </div>

              <div className="bg-ctp-crust p-2 text-ctp-base flex gap-1 flex-col justify-between  rounded-b-md">


                <div className='p-4'>
                  <Votes socket={socket} users={users} userId={userID} round={currentRound} votes={votes} />
                </div>



              </div>

              <div className="text-lg bg-slate-800 pl-4 mt-3 pr-2 py-2 text-slate-300 text-left rounded-t-md">
                Your vote
              </div>
              <div className='bg-ctp-crust p-4 text-ctp-base rounded-b-md'>

                <VoteArea socket={socket} room={room} vote={vote} />
              </div>

            </div>

            <div className='flex-1  min-w-64'>
              <div className="text-lg bg-slate-800 px-4 py-2 text-slate-300 text-left rounded-t-md">Players</div>
              <div className='bg-ctp-crust p-4 text-ctp-base rounded-b-md'>

                <UsersList socket={socket} users={users} userId={userID} />
              </div>

              <div className="text-lg bg-slate-800 px-4 py-2 text-slate-300 text-left rounded-t-md mt-3">Rounds</div>
              <div className='bg-ctp-crust p-4 text-ctp-base rounded-b-md'>

                <RoundsList socket={socket} rounds={rounds} users={users} userId={userID} />
              </div>

            </div>
          </div >
        </> : <RoomForm socket={socket} />
        }

        {
          showUserModal && <>
            <div className='absolute top-0 left-0 right-0 bottom-0 backdrop-blur-sm bg-opacity-50 bg-black flex justify-center items-center'>
              <dialog aria-modal='true' open={showUserModal} className='p-1 bg-slate-500 shadow-xl'>
                <div className='bg-slate-500 text-white text-xl'>User options</div>
                <UserForm socket={socket} user={user} onCloseDialog={() => setShowUserModal(false)} />
              </dialog>
            </div>
          </>
        }

      </> :
        (
          <Login socket={socket} />
        )
      }

    </>
  )
}

// eslint-disable-next-line @typescript-eslint/no-unused-vars
const Debug: FC<{ value: string | object | boolean | number | [] | undefined }> = ({ value }) => {
  return <pre>{JSON.stringify(value, null, 4)}</pre>
}



export default App
