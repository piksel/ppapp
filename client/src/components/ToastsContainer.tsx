import { FC } from 'react';
import { Toast, ToastType } from '../types/toasts';

const toastStyles: Record<ToastType, string> = {
    error: 'bg-red-500 border-red-950',
    info: ' bg-blue-500 border-blue-950'
}


export const ToastsContainer: FC<{ toasts: Toast[]; removeToast: (id: string) => void; }> = (props) => {
    const { toasts, removeToast } = props;

    return (
        <div
            className='absolute top-0 right-0 bottom-0 overflow-hidden w-96 flex flex-col justify-start gap-3 p-3 pointer-events-none'>
            {toasts.map(toast => <ToastComponent key={toast.id} toast={toast} removeToast={removeToast} />)}
        </div>
    );
};

const ToastComponent: FC<{ toast: Toast, removeToast: (id: string) => void; }> = ({ toast, removeToast }) => {
    return (
        <div key={toast.id}
            className={`text-white ${toastStyles[toast.type]} flex flex-col justify-between shadow-md border-2 rounded-md pointer-events-auto`}>

            <div className='flex bg-black bg-opacity-25 justify-between border-b-2 border-red-950'>
                <div className='px-1'>
                    {toast.type[0].toUpperCase() + toast.type.substring(1)}
                </div>
                <button className='px-2 border-l-2 border-red-950' onClick={() => removeToast(toast.id)}>close</button>
            </div>
            <div className='text-left py-1 px-2'> {toast.message}</div>
        </div>
    )
}