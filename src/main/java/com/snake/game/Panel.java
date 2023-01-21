package com.snake.game;

import java.awt.Color;

import javax.swing.JPanel;

import java.awt.Graphics;

public class Panel extends JPanel implements Grid {
    private int cellSize;
    //
    public Panel() {
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
        createGrid(g, 19, 19);
    }
    //
    @Override
    public void createGrid(Graphics g, int rows, int cols) {
        g.setColor(Color.GRAY);
        //
        for (int i = 0; i < rows; i++) {
            g.drawLine(0, i*cellSize, 576, i*cellSize);
        }
        for (int i = 0; i < cols; i++) {
            g.drawLine(i*cellSize, 0, i*cellSize, 576);
        }
    }
}
