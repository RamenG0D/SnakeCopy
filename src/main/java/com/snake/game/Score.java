package com.snake.game;

public class Score {
    private int score;
    public int getScore() {
        return score;
    }
    public void addScore(int score) {
        this.score += score;
    }
    public void reset() {
        score = 0;
    }
}
