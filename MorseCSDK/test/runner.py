#!/usr/bin/env python3
"""
Raspberry Pi Pico Benchmark Analysis Tool
This script analyzes benchmark results from the Pico benchmark suite and generates plots.
"""

import pandas as pd
import matplotlib.pyplot as plt
import seaborn as sns
import sys
import os
import argparse
from pathlib import Path

def load_data(filepath):
    """Load benchmark data from CSV file"""
    try:
        df = pd.read_csv(filepath)
        return df
    except Exception as e:
        print(f"Error loading file {filepath}: {e}")
        return None

def combine_results(input_dir, output_file):
    """Combine all benchmark CSV files into one"""
    print("Combining benchmark results...")
    
    combined_df = pd.DataFrame()
    
    # Find all CSV files in the input directory
    csv_files = list(Path(input_dir).glob("*.csv"))
    
    if not csv_files:
        print(f"No CSV files found in {input_dir}")
        return None
    
    # Read and combine each result file
    for file_path in csv_files:
        try:
            df = pd.read_csv(file_path)
            combined_df = pd.concat([combined_df, df], ignore_index=True)
            print(f"Added data from {file_path}")
        except Exception as e:
            print(f"Error reading {file_path}: {e}")
    
    # Save combined results
    if not combined_df.empty:
        combined_df.to_csv(output_file, index=False)
        print(f"Combined results saved to {output_file}")
        return combined_df
    else:
        print("No data collected.")
        return None

def plot_gpio_benchmark(df, output_dir):
    """Plot GPIO toggle benchmark results"""
    if 'gpio' not in df['task'].values:
        print("No GPIO benchmark data found")
        return
    
    gpio_df = df[df['task'] == 'gpio']
    
    # Plot average toggle time across iterations
    plt.figure(figsize=(10, 6))
    plt.plot(gpio_df['iteration'], gpio_df['avg_toggle_us'], 'o-', color='blue')
    plt.title('GPIO Toggle Performance')
    plt.xlabel('Iteration')
    plt.ylabel('Average Toggle Time (μs)')
    plt.grid(True, linestyle='--', alpha=0.7)
    plt.tight_layout()
    plt.savefig(os.path.join(output_dir, 'gpio_toggle_performance.png'), dpi=300)
    
    # Create histogram of toggle times
    plt.figure(figsize=(10, 6))
    sns.histplot(gpio_df['avg_toggle_us'], bins=20, kde=True)
    plt.title('Distribution of GPIO Toggle Times')
    plt.xlabel('Average Toggle Time (μs)')
    plt.ylabel('Frequency')
    plt.grid(True, linestyle='--', alpha=0.7)
    plt.tight_layout()
    plt.savefig(os.path.join(output_dir, 'gpio_toggle_histogram.png'), dpi=300)
    
    # Summary statistics
    print("GPIO Toggle Benchmark Statistics:")
    print(f"Average toggle time: {gpio_df['avg_toggle_us'].mean():.3f} μs")
    print(f"Min toggle time: {gpio_df['avg_toggle_us'].min():.3f} μs")
    print(f"Max toggle time: {gpio_df['avg_toggle_us'].max():.3f} μs")
    print(f"Std dev: {gpio_df['avg_toggle_us'].std():.3f} μs")

def plot_pwm_benchmark(df, output_dir):
    """Plot PWM setup benchmark results"""
    if 'pwm' not in df['task'].values:
        print("No PWM benchmark data found")
        return
    
    pwm_df = df[df['task'] == 'pwm']
    
    # Plot setup time across iterations
    plt.figure(figsize=(10, 6))
    plt.plot(pwm_df['iteration'], pwm_df['setup_time_us'], 'o-', color='green')
    plt.title('PWM Setup Performance')
    plt.xlabel('Iteration')
    plt.ylabel('Setup Time (μs)')
    plt.grid(True, linestyle='--', alpha=0.7)
    plt.tight_layout()
    plt.savefig(os.path.join(output_dir, 'pwm_setup_performance.png'), dpi=300)
    
    # Create histogram of setup times
    plt.figure(figsize=(10, 6))
    sns.histplot(pwm_df['setup_time_us'], bins=20, kde=True)
    plt.title('Distribution of PWM Setup Times')
    plt.xlabel('Setup Time (μs)')
    plt.ylabel('Frequency')
    plt.grid(True, linestyle='--', alpha=0.7)
    plt.tight_layout()
    plt.savefig(os.path.join(output_dir, 'pwm_setup_histogram.png'), dpi=300)
    
    # Summary statistics
    print("PWM Setup Benchmark Statistics:")
    print(f"Average setup time: {pwm_df['setup_time_us'].mean():.3f} μs")
    print(f"Min setup time: {pwm_df['setup_time_us'].min():.3f} μs")
    print(f"Max setup time: {pwm_df['setup_time_us'].max():.3f} μs")
    print(f"Std dev: {pwm_df['setup_time_us'].std():.3f} μs")

def plot_adc_benchmark(df, output_dir):
    """Plot ADC read benchmark results"""
    if 'adc' not in df['task'].values:
        print("No ADC benchmark data found")
        return
    
    adc_df = df[df['task'] == 'adc']
    
    # Plot average read time across iterations
    plt.figure(figsize=(10, 6))
    plt.plot(adc_df['iteration'], adc_df['avg_time_us'], 'o-', color='red')
    plt.title('ADC Read Performance')
    plt.xlabel('Iteration')
    plt.ylabel('Average Read Time (μs)')
    plt.grid(True, linestyle='--', alpha=0.7)
    plt.tight_layout()
    plt.savefig(os.path.join(output_dir, 'adc_read_performance.png'), dpi=300)
    
    # Create histogram of read times
    plt.figure(figsize=(10, 6))
    sns.histplot(adc_df['avg_time_us'], bins=20, kde=True)
    plt.title('Distribution of ADC Read Times')
    plt.xlabel('Average Read Time (μs)')
    plt.ylabel('Frequency')
    plt.grid(True, linestyle='--', alpha=0.7)
    plt.tight_layout()
    plt.savefig(os.path.join(output_dir, 'adc_read_histogram.png'), dpi=300)
    
    # Summary statistics
    print("ADC Read Benchmark Statistics:")
    print(f"Average read time: {adc_df['avg_time_us'].mean():.3f} μs")
    print(f"Min read time: {adc_df['avg_time_us'].min():.3f} μs")
    print(f"Max read time: {adc_df['avg_time_us'].max():.3f} μs")
    print(f"Std dev: {adc_df['avg_time_us'].std():.3f} μs")

def plot_interrupt_benchmark(df, output_dir):
    """Plot interrupt latency benchmark results"""
    if 'interrupt' not in df['task'].values:
        print("No interrupt benchmark data found")
        return
    
    int_df = df[df['task'] == 'interrupt']
    
    # Plot latency across iterations
    plt.figure(figsize=(10, 6))
    plt.plot(int_df['iteration'], int_df['latency_us'], 'o-', color='purple')
    plt.title('Interrupt Latency Performance')
    plt.xlabel('Iteration')
    plt.ylabel('Latency (μs)')
    plt.grid(True, linestyle='--', alpha=0.7)
    plt.tight_layout()
    plt.savefig(os.path.join(output_dir, 'interrupt_latency_performance.png'), dpi=300)
    
    # Create histogram of latencies
    plt.figure(figsize=(10, 6))
    sns.histplot(int_df['latency_us'], bins=20, kde=True)
    plt.title('Distribution of Interrupt Latencies')
    plt.xlabel('Latency (μs)')
    plt.ylabel('Frequency')
    plt.grid(True, linestyle='--', alpha=0.7)
    plt.tight_layout()
    plt.savefig(os.path.join(output_dir, 'interrupt_latency_histogram.png'), dpi=300)
    
    # Summary statistics
    print("Interrupt Latency Benchmark Statistics:")
    print(f"Average latency: {int_df['latency_us'].mean():.3f} μs")
    print(f"Min latency: {int_df['latency_us'].min():.3f} μs")
    print(f"Max latency: {int_df['latency_us'].max():.3f} μs")
    print(f"Std dev: {int_df['latency_us'].std():.3f} μs")

def plot_uart_benchmark(df, output_dir):
    """Plot UART transmission benchmark results"""
    if 'uart' not in df['task'].values:
        print("No UART benchmark data found")
        return
    
    uart_df = df[df['task'] == 'uart']
    
    # Plot transmission time by message size
    plt.figure(figsize=(10, 6))
    for bytes_size in uart_df['bytes'].unique():
        subset = uart_df[uart_df['bytes'] == bytes_size]
        plt.plot(subset['iteration'], subset['time_us'], 'o-', label=f'{bytes_size} bytes')
    
    plt.title('UART Transmission Time by Message Size')
    plt.xlabel('Iteration')
    plt.ylabel('Transmission Time (μs)')
    plt.legend()
    plt.grid(True, linestyle='--', alpha=0.7)
    plt.tight_layout()
    plt.savefig(os.path.join(output_dir, 'uart_transmission_time.png'), dpi=300)
    
    # Plot throughput by message size
    plt.figure(figsize=(10, 6))
    avg_throughput = uart_df.groupby('bytes')['bytes_per_sec'].mean().reset_index()
    plt.bar(avg_throughput['bytes'].astype(str), avg_throughput['bytes_per_sec'])
    plt.title('UART Throughput by Message Size')
    plt.xlabel('Message Size (bytes)')
    plt.ylabel('Throughput (bytes/sec)')
    plt.grid(True, linestyle='--', alpha=0.7)
    plt.tight_layout()
    plt.savefig(os.path.join(output_dir, 'uart_throughput.png'), dpi=300)
    
    # Summary statistics by message size
    print("UART Transmission Benchmark Statistics:")
    for bytes_size in sorted(uart_df['bytes'].unique()):
        subset = uart_df[uart_df['bytes'] == bytes_size]
        print(f"\nMessage size: {bytes_size} bytes")
        print(f"  Average transmission time: {subset['time_us'].mean():.3f} μs")
        print(f"  Average throughput: {subset['bytes_per_sec'].mean():.2f} bytes/sec")

def plot_combined_results(df, output_dir):
    """Create combined visualizations across benchmark types"""
    
    # Compare average execution times across benchmark types
    task_means = {}
    
    if 'gpio' in df['task'].values:
        gpio_df = df[df['task'] == 'gpio']
        task_means['GPIO Toggle'] = gpio_df['avg_toggle_us'].mean()
    
    if 'adc' in df['task'].values:
        adc_df = df[df['task'] == 'adc']
        task_means['ADC Read'] = adc_df['avg_time_us'].mean()
    
    if 'pwm' in df['task'].values:
        pwm_df = df[df['task'] == 'pwm']
        task_means['PWM Setup'] = pwm_df['setup_time_us'].mean()
    
    if 'interrupt' in df['task'].values:
        int_df = df[df['task'] == 'interrupt']
        task_means['Interrupt Latency'] = int_df['latency_us'].mean()
    
    if task_means:
        plt.figure(figsize=(12, 6))
        tasks = list(task_means.keys())
        times = list(task_means.values())
        
        bars = plt.bar(tasks, times)
        
        # Add value labels on top of each bar
        for bar in bars:
            height = bar.get_height()
            plt.text(bar.get_x() + bar.get_width()/2., height + 0.1,
                    f'{height:.2f}',
                    ha='center', va='bottom', rotation=0)
        
        plt.title('Average Execution Time by Benchmark Type')
        plt.ylabel('Time (μs)')
        plt.yscale('log')  # Log scale may help with widely different values
        plt.grid(True, axis='y', linestyle='--', alpha=0.7)
        plt.tight_layout()
        plt.savefig(os.path.join(output_dir, 'combined_benchmark_comparison.png'), dpi=300)

def main():
    parser = argparse.ArgumentParser(description='RP2040 Benchmark Analysis Tool')
    parser.add_argument('--input', '-i', default='/tmp', help='Directory containing benchmark CSV files')
    parser.add_argument('--output', '-o', default='benchmark_results', help='Output directory for plots')
    parser.add_argument('--combine_only', action='store_true', help='Only combine CSV files, don\'t generate plots')
    parser.add_argument('--plot_only', action='store_true', help='Only generate plots, don\'t combine CSV files')
    parser.add_argument('--combined_file', default='combined_results.csv', help='Name of the combined CSV file')
    
    args = parser.parse_args()
    
    # Create output directory if it doesn't exist
    output_dir = args.output
    os.makedirs(output_dir, exist_ok=True)
    
    # Set path for combined results
    combined_path = os.path.join(output_dir, args.combined_file)
    
    # Combine results or load existing
    if not args.plot_only:
        df = combine_results(args.input, combined_path)
    else:
        df = load_data(combined_path)
    
    if df is None:
        print("Error: No data to plot")
        sys.exit(1)
    
    # Stop here if combine_only flag is set
    if args.combine_only:
        print("CSV files combined. Exiting without generating plots.")
        sys.exit(0)
    
    # Create plots directory
    plots_dir = os.path.join(output_dir, 'plots')
    os.makedirs(plots_dir, exist_ok=True)
    
    # Generate plots
    plot_gpio_benchmark(df, plots_dir)
    plot_pwm_benchmark(df, plots_dir)
    plot_adc_benchmark(df, plots_dir)
    plot_interrupt_benchmark(df, plots_dir)
    plot_uart_benchmark(df, plots_dir)
    plot_combined_results(df, plots_dir)
    
    print(f"Analysis complete. Plots saved to {plots_dir}")

if __name__ == "__main__":
    main()