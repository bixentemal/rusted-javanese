package com.rusty;

import static java.lang.Math.random;

public class JavaJungleJuice {

    public static void main(String args[]) {
        long startTime = System.currentTimeMillis();
        int numberOfTrial = 100000;
        int numberOfIteration = 100000;
        int error = calculateError(numberOfTrial, numberOfIteration);
        long endTime = System.currentTimeMillis();
        long nbJniTransition = 0;
        System.out.println("Java -> Java -> Java : res = "+error+" Total execution time for JNI transitions (    ) = "+nbJniTransition+" \t\t: " + (endTime-startTime) + "ms");

    }
    public static int calculateError(int numberOfTrials, int numberOfIterations) {
        int error = 0;
        for (int i = 0; i < numberOfTrials; i++) {
            error += 50 - calc(numberOfIterations);
        }
        return error;
    }

    private static int calc(int n) {
        int sum = 0;
        for (int i = 0; i < n; i++) {
            int r = gen(n, i);
            sum += r;
        }
        return sum / n;
    }

    public static int gen(int m, int round) {
        return  m * round % 10;
    }
}



