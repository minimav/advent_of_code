import java.io.FileReader;
import java.io.FileNotFoundException;
import java.lang.StringBuilder;
import java.util.Scanner;
import java.util.List;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.Objects;
import java.util.HashSet;


class Position {
    private int row;
    private int column;

    public Position(int row, int column) {
        this.row = row; 
        this.column = column;
    }

    public int getRow() { return this.row; }
    public int getColumn() { return this.column; }

    public void setRow(int row) { this.row = row; }
    public void setColumn(int column) { this.column = column; }

    public String toString() {
        return String.format("Position(%d, %d)", this.row, this.column);
    }

    @Override
    public int hashCode() {
        return Objects.hash(row, column);
    }

    @Override
    public boolean equals(Object obj){
        if (this == obj) {
            return true;
        }
        if (obj == null || getClass() != obj.getClass()){
            return false; 
        }
        Position other = (Position) obj;
        return row == other.row && column == other.column;
    }
}

class Light {
    private Position position;
    private String direction;

    public Light(Position position, String direction) {
        this.position = position;
        this.direction = direction;
    }

    public Light(int row, int column, String direction) {
        this.position = new Position(row, column);
        this.direction = direction;
    }

    public int getRow() { return this.position.getRow(); }
    public int getColumn() { return this.position.getColumn(); }
    public String getDirection() { return this.direction; }

    public void setRow(int row) { this.position.setRow(row); }
    public void setColumn(int column) { this.position.setColumn(column); }
    public void setDirection(String direction) { this.direction = direction; }

    public void moveUp() {
        this.position.setRow(this.position.getRow() - 1);
    }

    public void moveDown() {
        this.position.setRow(this.position.getRow() + 1);
    }

    public void moveLeft() {
        this.position.setColumn(this.position.getColumn() - 1);
    }

    public void moveRight() {
        this.position.setColumn(this.position.getColumn() + 1);
    }

    public String toString() {
        return String.format("Light(%s, %s)", this.position, this.direction);
    }
}


public class Puzzle {

    private static String readFile(String filename) throws FileNotFoundException {
        try {
            Scanner in = new Scanner(new FileReader(filename));
            StringBuilder sb = new StringBuilder();
            while(in.hasNextLine()) {
                sb.append(in.nextLine());
                if (in.hasNextLine()) {
                    sb.append("\n");
                }
            }
            in.close();
            return sb.toString();
        } catch (FileNotFoundException e) {
            throw e;
        }
    }

    private static boolean notMirror(Light light, List<String> lines) {
        int row = light.getRow();
        int column = light.getColumn();
        
        char c = lines.get(row).charAt(column);

        if ((c == '\\') || (c == '/') || (c == '|') || (c == '-')) {
            return false;
        } else {
            return true;
        }
    }

    private static void printEnergised(int numRows, int numCols, HashSet<Position> energised) {
        StringBuilder sb = new StringBuilder();
        for (int row = 0; row < numRows; row++) {
            for (int column = 0; column < numCols; column++) {
                if (energised.contains(new Position(row, column))) {
                    sb.append("#");
                } else {
                    sb.append(".");
                }
            }
            sb.append("\n");
        }
        System.out.println(sb.toString());
    }

    private static int beam(String input, int startRow, int startColumn, String startDirection) {
        List<String> lines = Arrays.asList(input.split("\n"));

        int numRows = lines.size();
        int numCols = lines.get(0).length();
        
        Light initialLight = new Light(startRow, startColumn, startDirection);
        
        // Maintain set of energised positions - need to record all positions
        // along the light's path, not just mirror positions
        HashSet<Position> energised = new HashSet<>();
        energised.add(new Position(startRow, startColumn));

        // Maintain set of lights seen before to avoid reprocessing beams
        // travelling in the same direction - only need to store the mirror
        // positions here
        HashSet<String> seenBefore = new HashSet<>();
        seenBefore.add(initialLight.toString());

        List<Light> lights = new ArrayList<>();
        lights.add(initialLight);

        while (!lights.isEmpty()) {                    
            Light light = lights.remove(0);
            seenBefore.add(light.toString());
            energised.add(new Position(light.getRow(), light.getColumn()));
            
            // Move as far as you can go till you hit a mirror or grid edge
            if (light.getDirection() == "R") {
                while ((light.getColumn() < numCols - 1) && notMirror(light, lines)) {
                    light.moveRight();
                    energised.add(new Position(light.getRow(), light.getColumn()));
                }
                //System.out.println(String.format("RIGHT to %s", light));
            } else if (light.getDirection() == "L") {
                while ((light.getColumn() > 0) && notMirror(light, lines)) {
                    light.moveLeft();
                    energised.add(new Position(light.getRow(), light.getColumn()));
                }
                //System.out.println(String.format("LEFT to %s", light));
            } else if (light.getDirection() == "U") {
                while ((light.getRow() > 0) && notMirror(light, lines)) {
                    light.moveUp();
                    energised.add(new Position(light.getRow(), light.getColumn()));
                }
                //System.out.println(String.format("UP to %s", light));
            } else if (light.getDirection() == "D") {
                while ((light.getRow() < numRows - 1) && notMirror(light, lines)) {
                    light.moveDown();
                    energised.add(new Position(light.getRow(), light.getColumn()));
                }
                //System.out.println(String.format("DOWN to %s", light));
            }

            seenBefore.add(light.toString());

            if (notMirror(light, lines)) {
                // Non-mirror terminal position must be at edge of grid, already
                // added to energised
                assert(
                    light.getRow() == numRows - 1 ||
                    light.getColumn() == numCols - 1 ||
                    light.getRow() == 0 ||
                    light.getColumn() == 0
                );
                continue;
            } else {
                // Moved to mirror position, add to energised
                energised.add(new Position(light.getRow(), light.getColumn()));

                String direction = light.getDirection();

                List<Light> potentialLights = new ArrayList<>();
                char c = lines.get(light.getRow()).charAt(light.getColumn());
                if (c == '\\') {
                    if (direction == "R") {
                        light.setDirection("D");
                        light.moveDown();
                    } else if (direction == "L") {
                        light.setDirection("U");
                        light.moveUp();
                    } else if (direction == "U") {
                        light.setDirection("L");
                        light.moveLeft();
                    } else if (direction == "D") {
                        light.setDirection("R");
                        light.moveRight();
                    }
                    potentialLights.add(light);
                } else if (c == '/') {
                    if (direction == "R") {
                        light.setDirection("U");
                        light.moveUp();
                    } else if (direction == "L") {
                        light.setDirection("D");
                        light.moveDown();
                    } else if (direction == "U") {
                        light.setDirection("R");
                        light.moveRight();
                    } else if (direction == "D") {
                        light.setDirection("L");
                        light.moveLeft();
                    }
                    potentialLights.add(light);
                } else if (c == '|') {
                    if (direction == "U") {
                        light.moveUp();
                        potentialLights.add(light);
                    } else if (direction == "D") {
                        light.moveDown();
                        potentialLights.add(light);
                    } else {
                        potentialLights.add(new Light(light.getRow() - 1, light.getColumn(), "U"));
                        potentialLights.add(new Light(light.getRow() + 1, light.getColumn(), "D"));
                    }
                } else if (c == '-') {
                    if (direction == "L") {
                        light.moveLeft();
                        potentialLights.add(light);
                    } else if (direction == "R") {
                        light.moveRight();
                        potentialLights.add(light);
                    } else {
                        potentialLights.add(new Light(light.getRow(), light.getColumn() - 1, "L"));
                        potentialLights.add(new Light(light.getRow(), light.getColumn() + 1, "R"));
                    }
                }
                
                // For the potentials, check the position is within the grid and
                // that we haven't processed that location and direction before
                for (Light newLight : potentialLights) {
                    int row = newLight.getRow();
                    int column = newLight.getColumn();

                    boolean inBounds = (row >= 0) && (row < numRows) && (column >= 0) && (column < numCols);
                    boolean alreadySeen = seenBefore.contains(newLight.toString());

                    if (inBounds && !alreadySeen) {
                        seenBefore.add(newLight.toString());
                        lights.add(newLight);
                    }
                }
            }
        }
        return energised.size();
    }

    private static void part1(String input) {
        System.out.println(beam(input, 0, 0, "R"));
    }

    private static void part2(String input) {    
        List<String> lines = Arrays.asList(input.split("\n"));
        int numRows = lines.size();
        int numCols = lines.get(0).length();

        int currentBest = 0;
        // Left and right edges
        for (int row = 0; row < numRows; row++) {
            int rBeamSize = beam(input, row, 0, "R");
            if (rBeamSize > currentBest) {
                System.out.println(String.format("New best: %d (%d %d) R", rBeamSize, row, 0));
                currentBest = rBeamSize;
            }
            int lBeamSize = beam(input, row, numCols - 1, "L");
            if (lBeamSize > currentBest) {
                System.out.println(String.format("New best: %d (%d %d) L", lBeamSize, row, numCols - 1));
                currentBest = lBeamSize;
            }
        }
        // Top and bottom edges
        for (int column = 0; column < numCols; column++) {
            int dBeamSize = beam(input, 0, column, "D");
            if (dBeamSize > currentBest) {
                System.out.println(String.format("New best: %d (%d %d) D", dBeamSize, 0, column));
                currentBest = dBeamSize;
            }
            int uBeamSize = beam(input, numRows - 1, column, "U");
            if (uBeamSize > currentBest) {
                System.out.println(String.format("New best: %d (%d %d) U", uBeamSize, numRows - 1, column));
                currentBest = uBeamSize;
            }
        }
        System.out.println(currentBest);
    }

    public static void main(String[] args) {
        try {
            String example = readFile("../puzzle_16/example.txt");
            part1(example);
            part2(example);
        } catch (FileNotFoundException e) {
            System.out.print("Could not find example.txt");
        }

        try {
            String input = readFile("../puzzle_16/input.txt");
            part1(input);
            part2(input);
        } catch (FileNotFoundException e) {
            System.out.print("Could not find input.txt");
        }   
    }
}