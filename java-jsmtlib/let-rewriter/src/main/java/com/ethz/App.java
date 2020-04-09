package com.ethz;

import org.smtlib.sexpr.Parser;
import org.smtlib.impl.Factory;
import org.smtlib.IParser.IFactory;
import org.smtlib.IParser;
import java.io.File;
import org.smtlib.*;
import org.smtlib.SMT;
import org.smtlib.ICommand;
import org.smtlib.SMT.Configuration;
import org.smtlib.impl.Pos.Source;
import org.smtlib.solvers.Solver_test;
/**
 * Hello world!
 *
 */
public class App 
{
    public static void main( String[] args ) {}

    public static void test() {
        System.out.println( "Hello World!" );
        SMT smt = new SMT();
        Configuration cfg = smt.smtConfig;
        Solver_test solver = new Solver_test(cfg, null);
        IFactory fac = cfg.smtFactory;

        File f = new File("bug272.minimized.smtv1.smt2");
        try {
            Source s = new Source(cfg, f);
            IParser p = fac.createParser(cfg, s);
            while (!p.isEOD()) {
                ICommand cmd = p.parseCommand();
                if (cmd != null) {
                    cmd.execute(solver);
                }
            }

        } catch (Exception e) {
                throw new RuntimeException("fail");
        }

        System.out.println( "Hello World!" );

    }
}
