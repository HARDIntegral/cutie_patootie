export function fire_confetti() {
    confetti({
        particleCount: 150,
        spread: 90,
        origin: { y: 0.6 },
        colors: ['#ff5ecf', '#ffb6d9', '#ffc0cb'],
    });
}
