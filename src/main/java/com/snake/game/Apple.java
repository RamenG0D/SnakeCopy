package com.snake.game;

import javax.swing.JLabel;
import java.awt.Color;
import java.awt.Graphics;

public class Apple extends JLabel {
    private int x;
    private int y;
    //
    public Apple(int pos) {
        this.x = pos;
        this.y = pos;
    }
    //
    public int getX() {
        return x;
    }
    public int getY() {
        return y;
    }
    public void setRandomPos(int randPos) {
        this.x = randPos;
        this.y = randPos;
    }
    @Override
    public void paint(Graphics g) {
        super.paint(g);
        //
        draw(g);
    }
    public void draw(Graphics g) {
        g.setColor(Color.RED);
        g.fillRoundRect(x*32, y*32, 32, 32, 100, 100);
    }
}