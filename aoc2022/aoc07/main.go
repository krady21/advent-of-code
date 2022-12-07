package main

import (
	"fmt"
	"os"
	"strconv"
	"strings"
)

type File struct {
	Size int
}

type Dir struct {
	Parent   *Dir
	Children map[string]*Dir
	Files    map[string]*File
}

func NewDir(parent *Dir) *Dir {
	return &Dir{
		Parent:   parent,
		Children: make(map[string]*Dir),
		Files:    make(map[string]*File),
	}
}

func NewFile(size int) *File {
	return &File{
		Size: size,
	}
}

var root *Dir = NewDir(nil)

func parse(input string) {
	lines := strings.Split(input, "\n")

	var currentDir *Dir
	for _, line := range lines {
		line = strings.TrimSpace(line)

		split := strings.Split(line, " ")

		if line[0] == '$' { // cmd
			if split[1] != "cd" {
				continue
			}

			if split[2] == "/" {
				currentDir = root
			} else if split[2] == ".." {
				currentDir = currentDir.Parent
			} else {
				currentDir = currentDir.Children[split[2]]
			}
		} else { // files or directories
			if split[0] == "dir" { // dirs
				currentDir.Children[split[1]] = NewDir(currentDir)
			} else { // files
				fileSize, _ := strconv.Atoi(split[0])
				currentDir.Files[split[1]] = NewFile(fileSize)
			}
		}
	}
}

func GetDirSize(d *Dir) int {
	var currentDirSize int
	for _, f := range d.Files {
		currentDirSize += f.Size
	}

	for _, child := range d.Children {
		currentDirSize += GetDirSize(child)
	}

	return currentDirSize
}

func checkSize(d *Dir, sum *int) {
	size := GetDirSize(d)
	if size <= 100000 {
		*sum += size
	}
	for _, child := range d.Children {
		checkSize(child, sum)
	}
}

func findSmallestToDelete(d *Dir, min *int, needToFree int) int {
	size := GetDirSize(d)
	if size < *min && size > needToFree {
		*min = size
	}

	for _, child := range d.Children {
		findSmallestToDelete(child, min, needToFree)
	}

	return *min
}

func task1() int {
	var sum int
	checkSize(root, &sum)
	return sum
}

func task2() int {
	used := GetDirSize(root)
	totalDisk := 70000000
	totalSpaceNeeded := 30000000

	unused := totalDisk - used
	needToFree := totalSpaceNeeded - unused

	return findSmallestToDelete(root, &used, needToFree)
}

func main() {
	f, _ := os.ReadFile("input.txt")
	file := string(f)
	file = strings.TrimSpace(file)

	parse(file)
	result1 := task1()
	result2 := task2()

	fmt.Println("Task 1 result is:", result1)
	fmt.Println("Task 2 result is:", result2)
}
