import java.io.FileReader;
import java.io.FileNotFoundException;
import java.lang.StringBuilder;
import java.util.Scanner;

public class Puzzle {

    private static String readFile(String filename) throws FileNotFoundException {
        try {
            Scanner in = new Scanner(new FileReader(filename));
            StringBuilder sb = new StringBuilder();
            while(in.hasNextLine()) {
                sb.append(in.nextLine());
            }
            in.close();
            return sb.toString();
        } catch (FileNotFoundException e) {
            throw e;
        }
    }

    private static void puzzle(String input) {
        System.out.println(input);
    }

    public static void main(String[] args) {
        try {
            String example = readFile("../example.txt");
            puzzle(example);
        } catch (FileNotFoundException e) {
            System.out.print("Could not find example.txt");
        }

        try {
            String input = readFile("../input.txt");
            puzzle(input);
        } catch (FileNotFoundException e) {
            System.out.print("Could not find input.txt");
        }
        
    }
}