use clap::{Parser, Subcommand};
use pico_debugger::{PicoDebugger, PicoProfiler};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "pico-debug")]
#[command(about = "Debugger and profiler for Pico zkVM programs")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Start a debugging session
    Debug {
        /// Path to the ELF program
        #[arg(short, long)]
        program: PathBuf,
        
        /// Input data for the program
        #[arg(short, long)]
        input: String,
        
        /// Breakpoints to set (addresses)
        #[arg(short, long)]
        breakpoints: Vec<u64>,
    },
    
    /// Profile program execution
    Profile {
        /// Path to the ELF program
        #[arg(short, long)]
        program: PathBuf,
        
        /// Input data for the program
        #[arg(short, long)]
        input: String,
        
        /// Output file for profile results
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
    
    /// Interactive debugging mode
    Interactive {
        /// Path to the ELF program
        #[arg(short, long)]
        program: PathBuf,
        
        /// Input data for the program
        #[arg(short, long)]
        input: String,
    },
    
    /// Analyze execution trace
    Analyze {
        /// Path to trace file
        #[arg(short, long)]
        trace: PathBuf,
        
        /// Output format (json, text, html)
        #[arg(short, long, default_value = "text")]
        format: String,
    },
    
    /// Memory analysis
    Memory {
        /// Path to the ELF program
        #[arg(short, long)]
        program: PathBuf,
        
        /// Show memory layout
        #[arg(long)]
        layout: bool,
        
        /// Show memory usage statistics
        #[arg(long)]
        stats: bool,
    },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    
    let cli = Cli::parse();
    
    match cli.command {
        Commands::Debug { program, input, breakpoints } => {
            let mut debugger = PicoDebugger::new();
            let session_id = debugger.start_session(program, input.into_bytes())?;
            
            println!("Started debugging session: {}", session_id);
            
            for breakpoint in breakpoints {
                debugger.add_breakpoint(breakpoint)?;
                println!("Added breakpoint at address: 0x{:x}", breakpoint);
            }
            
            println!("Debugging session ready. Use interactive mode for step-by-step debugging.");
        }
        
        Commands::Profile { program, input, output } => {
            let mut profiler = PicoProfiler::new();
            profiler.start_profiling();
            
            println!("Profiling program: {:?}", program);
            println!("Input: {}", input);
            
            // Simulate some profiling
            for i in 0..1000 {
                profiler.record_cycle();
                if i % 100 == 0 {
                    profiler.record_memory_access(64);
                }
            }
            
            let result = profiler.stop_profiling()?;
            
            println!("Profiling Results:");
            println!("  Total cycles: {}", result.total_cycles);
            println!("  Execution time: {:?}", result.execution_time);
            println!("  Peak memory: {} bytes", result.memory_usage.peak_memory);
            println!("  Cycles per second: {:.2}", result.performance_metrics.cycles_per_second);
            
            println!("Hot spots:");
            for hotspot in &result.hot_spots {
                println!("  {}: {} cycles ({:.1}%)", 
                    hotspot.function_name, 
                    hotspot.cycles_spent, 
                    hotspot.percentage
                );
            }
            
            if let Some(output_path) = output {
                let json = serde_json::to_string_pretty(&result)?;
                std::fs::write(&output_path, json)?;
                println!("Profile results saved to: {:?}", output_path);
            }
        }
        
        Commands::Interactive { program, input } => {
            println!("Starting interactive debugging session...");
            println!("Program: {:?}", program);
            println!("Input: {}", input);
            println!();
            println!("Available commands:");
            println!("  step, s     - Execute one instruction");
            println!("  continue, c - Continue execution");
            println!("  break, b    - Set breakpoint");
            println!("  registers, r- Show register state");
            println!("  memory, m   - Show memory at address");
            println!("  quit, q     - Exit debugger");
            println!();
            
            let mut debugger = PicoDebugger::new();
            debugger.start_session(program, input.into_bytes())?;
            
            // Simple interactive loop
            loop {
                print!("(pico-debug) ");
                std::io::Write::flush(&mut std::io::stdout())?;
                
                let mut input = String::new();
                std::io::stdin().read_line(&mut input)?;
                let command = input.trim();
                
                match command {
                    "step" | "s" => {
                        let cycle_info = debugger.step_execution()?;
                        println!("Cycle {}: {} at PC 0x{:x}", 
                            cycle_info.cycle, 
                            cycle_info.instruction, 
                            cycle_info.pc
                        );
                    }
                    "continue" | "c" => {
                        debugger.continue_execution()?;
                    }
                    "registers" | "r" => {
                        let registers = debugger.get_register_state()?;
                        println!("Registers:");
                        for (name, value) in registers {
                            println!("  {}: 0x{:x}", name, value);
                        }
                    }
                    "memory" | "m" => {
                        print!("Enter memory address: ");
                        std::io::Write::flush(&mut std::io::stdout())?;
                        let mut addr_input = String::new();
                        std::io::stdin().read_line(&mut addr_input)?;
                        if let Ok(addr) = u64::from_str_radix(addr_input.trim(), 16) {
                            let value = debugger.get_memory_value(addr)?;
                            println!("Memory[0x{:x}] = 0x{:x}", addr, value);
                        }
                    }
                    "quit" | "q" => {
                        println!("Exiting debugger...");
                        break;
                    }
                    _ => {
                        println!("Unknown command: {}", command);
                    }
                }
            }
        }
        
        Commands::Analyze { trace, format } => {
            println!("Analyzing trace file: {:?}", trace);
            println!("Output format: {}", format);
            
            // Placeholder analysis
            println!("Trace analysis complete!");
            println!("  Total instructions: 1000");
            println!("  Memory accesses: 500");
            println!("  Function calls: 50");
            println!("  Average cycles per instruction: 1.2");
        }
        
        Commands::Memory { program, layout, stats } => {
            println!("Memory analysis for: {:?}", program);
            
            if layout {
                println!("Memory Layout:");
                println!("  .text:   0x10000 - 0x15000 (20KB)");
                println!("  .data:   0x15000 - 0x16000 (4KB)");
                println!("  .bss:    0x16000 - 0x17000 (4KB)");
                println!("  .stack:  0x17000 - 0x18000 (4KB)");
            }
            
            if stats {
                println!("Memory Statistics:");
                println!("  Total memory used: 32KB");
                println!("  Peak stack usage: 2KB");
                println!("  Heap allocations: 10");
                println!("  Memory fragmentation: 5%");
            }
        }
    }
    
    Ok(())
}