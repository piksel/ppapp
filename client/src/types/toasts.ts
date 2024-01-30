export type ToastType = 'error' | 'info';
export interface Toast {
    id: string;
    type: ToastType;
    message: string;
}