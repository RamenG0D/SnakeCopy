package com.snake.game;

import java.awt.Color;

import javax.swing.JPanel;

import java.awt.Graphics;

public class Panel extends JPanel implements Grid {
    private Snek snek;
    //
    public Panel(Snek snek) {
        this.snek = snek;
        //
        this.setSize(600, 600);
        this.setFocusable(false);
        this.setBackground(Color.BLACK);
        //
    }
    @Override
    public void paint(Graphics g) {
        super.paint(g);
        //
        draw(g);
    }
    public void draw(Graphics g) {
        createGrid(g, 19, 19, 32, 576);
    }
    //
    @Override
    public void createGrid(Graphics g, int rows, int cols, int cellSize, int windowSize) {
        g.setColor(Color.GRAY);
        //
        for (int i = 0; i < rows; i++) {
            g.drawLine(0, i*cellSize, windowSize, i*cellSize);
        }
        for (int i = 0; i < cols; i++) {
            g.drawLine(i*cellSize, 0, i*cellSize, windowSize);
        }
    }
}
