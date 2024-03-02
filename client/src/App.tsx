import "./App.css";
import { useSocket } from "./socket";

import { FC, useMemo, useState } from "react";
import { Login } from "./components/Login";
import { RoomForm } from "./components/RoomForm";
import { RoundsList } from "./components/RoundsList";
import { ToastsContainer } from "./components/ToastsContainer";
import { UserForm } from "./components/UserForm";
import { UsersList } from "./components/UsersList";
import { VoteArea } from "./components/VoteArea";
import { Votes } from "./components/Votes";
import { Result } from "./types";
import { RoundOpts } from "./types/ppapi/RoundOpts";
import { Toast, ToastType } from "./types/toasts";

function App() {
  const [showUserModal, setShowUserModal] = useState(false);
  const {
    socket,
    userID,
    room,
    user,
    users,
    rounds,
    votes,
    vote,
    currentRound,
  } = useSocket();

  const [roundType, setRoundType] = useState("mood");
  const roundOpts: RoundOpts = useMemo(() => {
    if (roundType === "mood") {
      return {
        anonymous: true,
        candidates: Array.from(Array(20), (_, i) => `${i * 5}`),
        max_votes: 1,
      };
    }
    return {
      anonymous: false,
      candidates: [],
      max_votes: 1,
    } as RoundOpts;
  }, [roundType]);

  const [toasts, setToasts] = useState<Toast[]>([]);

  const addToast = (message: string, type: ToastType) => {
    setToasts((curr) => [...curr, { message, type, id: crypto.randomUUID() }]);
  };

  const removeToast = (id: string) => {
    setToasts((curr) => curr.filter((t) => t.id !== id));
  };

  const handleResult = (r: Result) => {
    if (r.type === "Error") {
      addToast(r.error, "error");
    }
  };

  const editCurrentRound = () => {
    /* TODO */
  };

  return (
    <>
      <ToastsContainer toasts={toasts} removeToast={removeToast} />
      {user ? (
        <>
          <div className="flex  justify-between gap-5">
            <div className="flex-1">
              {room && (
                <>
                  <div className="flex flex-row gap-3 rounded-t-md bg-slate-800 px-4 py-2 text-left text-lg text-slate-300">
                    <div className="text-left text-xl text-slate-300">Room</div>

                    <button
                      className="text-sm text-white text-opacity-50 underline"
                      onClickCapture={() => {}}
                    >
                      Edit
                    </button>
                  </div>
                  <div className="bg-ctp-crust flex justify-between rounded-b-md px-4 py-1 text-white">
                    <div className="text-left text-4xl">{room.name}</div>
                    <div className="rounded-md bg-amber-700 px-4 py-1 text-right text-xl text-white">
                      {room.game}
                    </div>
                  </div>
                </>
              )}
            </div>

            <div className="bg-ctp-crust flex w-64 justify-stretch rounded-md p-3">
              <button
                className="flex flex-auto items-center gap-4 rounded-xl bg-slate-600 p-2"
                type="button"
                onClick={() => setShowUserModal(true)}
              >
                <div
                  className="h-12 w-12 rounded-xl bg-slate-200"
                  style={{
                    background: `url(https://gravatar.com/avatar/${user.avatar}?d=identicon)`,
                    backgroundSize: "cover",
                  }}
                ></div>
                <div className="text-left text-xl text-slate-300">
                  {user.name}
                </div>
              </button>
            </div>
          </div>

          {room ? (
            <>
              <div className="mt-3 flex gap-5">
                <div className="flex-auto">
                  <div className="rounded-t-md bg-slate-800 py-2 pl-4 pr-2 text-left text-lg text-slate-300">
                    <div className="flex items-center justify-between">
                      <h3 className=" text-left text-slate-300">
                        <span className="text-xl">
                          {currentRound?.name ?? ""}
                        </span>
                        <button
                          className="ml-2 text-sm text-white text-opacity-50 underline"
                          onClickCapture={() => editCurrentRound()}
                        >
                          Edit
                        </button>
                      </h3>
                      {!currentRound || currentRound.flipped ? (
                        <>
                          <div>
                            <select
                              onChange={(e) => setRoundType(e.target.value)}
                              value={roundType}
                              className="bg-ctp-text text-ctp-base placeholder-ctp-subtext0 rounded-l-md p-1"
                            >
                              <option value="mood">Mood check</option>
                              <option value="thing-sug">
                                Thing suggestions
                              </option>
                              <option value="thing-vote">Thing voting</option>
                            </select>
                            <button
                              className={`rounded-r-md bg-slate-600 px-6 py-1 text-white`}
                              onClickCapture={() =>
                                socket.emit(
                                  "new round",
                                  room.roomID,
                                  roundOpts,
                                  handleResult,
                                )
                              }
                            >
                              New round
                            </button>
                          </div>
                        </>
                      ) : (
                        <button
                          className={`rounded-md bg-slate-600 px-6 py-1 text-white`}
                          onClickCapture={() =>
                            socket.emit("end vote", room.roomID, handleResult)
                          }
                        >
                          Reveal cards
                        </button>
                      )}
                    </div>
                  </div>

                  <div className="bg-ctp-crust text-ctp-base flex flex-col justify-between gap-1 rounded-b-md  p-2">
                    <div className="p-4">
                      {currentRound ? (
                        <Votes
                          socket={socket}
                          users={users}
                          userId={userID}
                          round={currentRound}
                          votes={votes}
                        />
                      ) : (
                        <em className="text-gray-500">
                          no round has been started yet
                        </em>
                      )}
                    </div>
                  </div>

                  <div className="mt-3 rounded-t-md bg-slate-800 py-2 pl-4 pr-2 text-left text-lg text-slate-300">
                    Your vote
                  </div>
                  <div className="bg-ctp-crust text-ctp-base rounded-b-md p-4">
                    {currentRound ? (
                      <VoteArea
                        socket={socket}
                        room={room}
                        vote={vote}
                        candidates={currentRound.candidates}
                      />
                    ) : (
                      <em className="text-gray-500">
                        no round has been started yet
                      </em>
                    )}
                  </div>
                </div>

                <div className="min-w-64  flex-1">
                  <div className="rounded-t-md bg-slate-800 px-4 py-2 text-left text-lg text-slate-300">
                    Players
                  </div>
                  <div className="bg-ctp-crust text-ctp-base rounded-b-md p-4">
                    <UsersList socket={socket} users={users} userId={userID} />
                  </div>

                  <div className="mt-3 rounded-t-md bg-slate-800 px-4 py-2 text-left text-lg text-slate-300">
                    Rounds
                  </div>
                  <div className="bg-ctp-crust text-ctp-base rounded-b-md p-4">
                    <RoundsList
                      socket={socket}
                      rounds={rounds}
                      users={users}
                      userId={userID}
                    />
                  </div>
                </div>
              </div>
            </>
          ) : (
            <RoomForm socket={socket} />
          )}

          {showUserModal && (
            <>
              <div className="absolute bottom-0 left-0 right-0 top-0 flex items-center justify-center bg-black bg-opacity-50 backdrop-blur-sm">
                <dialog
                  aria-modal="true"
                  open={showUserModal}
                  className="bg-slate-500 p-1 shadow-xl"
                >
                  <div className="bg-slate-500 text-xl text-white">
                    User options
                  </div>
                  <UserForm
                    socket={socket}
                    user={user}
                    onCloseDialog={() => setShowUserModal(false)}
                  />
                </dialog>
              </div>
            </>
          )}
        </>
      ) : (
        <Login socket={socket} />
      )}
    </>
  );
}

// eslint-disable-next-line @typescript-eslint/no-unused-vars
const Debug: FC<{
  value: string | object | boolean | number | [] | undefined;
}> = ({ value }) => {
  return <pre>{JSON.stringify(value, null, 4)}</pre>;
};

export default App;
