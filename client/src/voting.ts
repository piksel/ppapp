const namedScores = ['coffee', 'unknown', 'infinite'];
const numScores = [1, 2, 4, 8, 16].map(s => s + '');
export const scores = [...namedScores, ...numScores];

export const scoreMojis: Record<string, string> = {
    'coffee': '☕️',
    'unknown': '❓',
    'infinite': '♾️',
    // ...Object.fromEntries(numScores.map(s => [s, s])),
    'picking': '🤔',
    'cardback': '🤫'
}

export const getScoreMoji = (candidate: string) => {
    const known = scoreMojis[candidate];
    return known ?? candidate;
}